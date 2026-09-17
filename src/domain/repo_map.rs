use super::symbol_graph::{PerProjectSymbolGraph, SymbolNode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoMapReport {
    pub total_symbols: usize,
    pub total_files: usize,
    pub estimated_tokens: usize,
    pub map_content: String,
}

pub struct RepoMapGenerator;

/// A single noisy file must not be able to consume the whole token budget and push
/// the architecturally interesting files past the truncation point.
const MAX_SYMBOLS_PER_FILE: usize = 25;

/// Characters held back from the body budget for the trailing omission notice.
const OMISSION_TRAILER_RESERVE: usize = 160;

impl RepoMapGenerator {
    /// Builds the map in memory only. Scanning someone else's repository must not
    /// leave anything behind on their disk; writing it out is [`Self::export`],
    /// which the caller invokes only when the user asked for a file.
    #[must_use]
    pub fn generate(workspace_root: &Path, max_tokens: usize) -> RepoMapReport {
        let symbols = PerProjectSymbolGraph::harvest_symbols(workspace_root);
        let total_symbols = symbols.len();
        let ranked_files = Self::rank_files_by_interface_weight(symbols);
        let total_files = ranked_files.len();

        let workspace_name = workspace_root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Active Project");

        let mut output = String::new();
        output.push_str("=== GODKILLER ZERO : PROJECT RADAR MAP ===\n");
        output.push_str(&format!(
            "Workspace: {} | Total files: {} | Symbols indexed: {}\n\n",
            workspace_name, total_files, total_symbols
        ));
        // Reserve room for the omission trailer so honouring the budget does not
        // itself push the map over it and trigger a hard cut.
        let body_char_budget = max_tokens
            .saturating_mul(4)
            .saturating_sub(output.len() + OMISSION_TRAILER_RESERVE);
        output.push_str(&Self::render_ranked_files(ranked_files, body_char_budget));

        // Token estimation heuristic (~4 chars per token for code/identifiers)
        let estimated_tokens = (output.len() / 4).max(1);
        let final_content = Self::apply_token_budget(output, estimated_tokens, max_tokens);

        RepoMapReport {
            total_symbols,
            total_files,
            estimated_tokens,
            map_content: final_content,
        }
    }

    /// Writes a generated map to `destination`, creating parent directories as
    /// needed. Only ever called when the user named an output file.
    pub fn export(destination: &Path, content: &str) -> std::io::Result<PathBuf> {
        if let Some(parent_dir) = destination
            .parent()
            .filter(|dir| !dir.as_os_str().is_empty())
        {
            fs::create_dir_all(parent_dir)?;
        }
        fs::write(destination, content)?;
        Ok(destination.to_path_buf())
    }

    /// Orders files so the ones exposing the most public surface are emitted first,
    /// because those survive truncation and are what a reader needs to orient.
    fn rank_files_by_interface_weight(symbols: Vec<SymbolNode>) -> Vec<(String, Vec<SymbolNode>)> {
        let mut grouped_files: BTreeMap<String, Vec<SymbolNode>> = BTreeMap::new();
        for symbol in symbols {
            grouped_files
                .entry(symbol.relative_path.clone())
                .or_default()
                .push(symbol);
        }

        let mut ranked_files: Vec<(String, Vec<SymbolNode>)> = grouped_files.into_iter().collect();
        ranked_files.sort_by(|left, right| {
            Self::interface_weight(&right.1)
                .cmp(&Self::interface_weight(&left.1))
                .then_with(|| left.0.cmp(&right.0))
        });
        ranked_files
    }

    fn interface_weight(symbols: &[SymbolNode]) -> usize {
        symbols
            .iter()
            .filter(|symbol| {
                matches!(
                    symbol.symbol_kind.as_str(),
                    "Public Function"
                        | "Public Class"
                        | "Public Struct"
                        | "Class"
                        | "Struct"
                        | "Trait"
                        | "Enum"
                        | "Interface"
                )
            })
            .count()
    }

    /// Stops at a file boundary rather than mid-declaration, because a map cut in
    /// half mid-file reads as corruption and costs the reader more than it saves.
    fn render_ranked_files(
        ranked_files: Vec<(String, Vec<SymbolNode>)>,
        char_budget: usize,
    ) -> String {
        let mut rendered = String::new();
        let mut omitted_files = 0;
        for (relative_path, file_symbols) in ranked_files {
            let file_block = Self::render_single_file(&relative_path, file_symbols);
            if rendered.len() + file_block.len() > char_budget {
                omitted_files += 1;
                continue;
            }
            rendered.push_str(&file_block);
        }
        if omitted_files > 0 {
            rendered.push_str(&format!(
                "... {} lower-signal files omitted to fit the token budget\n",
                omitted_files
            ));
        }
        rendered
    }

    fn render_single_file(relative_path: &str, mut file_symbols: Vec<SymbolNode>) -> String {
        let mut block = format!("{}:\n", relative_path);
        file_symbols.sort_by_key(|symbol| symbol.line_number);
        let omitted_count = file_symbols.len().saturating_sub(MAX_SYMBOLS_PER_FILE);
        for symbol in file_symbols.iter().take(MAX_SYMBOLS_PER_FILE) {
            block.push_str(&format!(
                "  │ L{:<4} {:<16} {}\n",
                symbol.line_number, symbol.symbol_kind, symbol.symbol_name
            ));
        }
        if omitted_count > 0 {
            block.push_str(&format!(
                "  │ ... {} further declarations in this file\n",
                omitted_count
            ));
        }
        block
    }

    fn apply_token_budget(output: String, estimated_tokens: usize, max_tokens: usize) -> String {
        if estimated_tokens <= max_tokens || max_tokens <= 100 {
            return output;
        }
        let char_limit = max_tokens * 4;
        let mut truncated = output.chars().take(char_limit).collect::<String>();
        truncated.push_str("\n... [RADAR MAP TRUNCATED TO FIT TOKEN BUDGET]\n");
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_repo_map_generation() {
        let temp_dir_path = std::env::temp_dir().join(format!(
            "godkiller_repo_map_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let src_dir = temp_dir_path.join("src");
        fs::create_dir_all(&src_dir).expect("Failed to create src");

        let test_file = src_dir.join("lib.rs");
        let mut f = File::create(&test_file).expect("Failed to create lib.rs");
        writeln!(
            f,
            r#"
pub struct UserAccount {{
    pub id: u64,
}}

pub fn authenticate_user(id: u64) -> bool {{
    id > 0
}}

fn internal_helper() {{
}}
"#
        )
        .expect("Write to lib.rs");

        let report = RepoMapGenerator::generate(&temp_dir_path, 2000);
        assert!(report.total_symbols >= 2);
        assert!(report.total_files >= 1);
        assert!(report.map_content.contains("UserAccount"));
        assert!(report.map_content.contains("authenticate_user"));

        assert!(
            !temp_dir_path.join(".gemini").exists(),
            "Generating a map must not create directories inside the scanned workspace"
        );

        let explicit_destination = temp_dir_path.join("out").join("REPO_MAP.md");
        let written_file = RepoMapGenerator::export(&explicit_destination, &report.map_content)
            .expect("Explicit export must succeed");
        assert_eq!(written_file, explicit_destination);
        assert!(fs::read_to_string(&explicit_destination)
            .expect("Exported map must be readable")
            .contains("UserAccount"));

        let _ = fs::remove_dir_all(&temp_dir_path);
    }
}
