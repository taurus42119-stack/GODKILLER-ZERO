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
    pub output_file_path: Option<PathBuf>,
}

pub struct RepoMapGenerator;

impl RepoMapGenerator {
    #[must_use]
    pub fn generate(workspace_root: &Path, max_tokens: usize) -> RepoMapReport {
        let symbols = PerProjectSymbolGraph::harvest_symbols(workspace_root);
        let total_symbols = symbols.len();

        // Group by relative path using BTreeMap for deterministic, sorted file order
        let mut file_map: BTreeMap<String, Vec<SymbolNode>> = BTreeMap::new();
        for s in symbols {
            file_map.entry(s.relative_path.clone()).or_default().push(s);
        }
        let total_files = file_map.len();

        let workspace_name = workspace_root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Active Project");

        let mut output = String::new();
        output.push_str("=== GODKILLER ZERO : PROJECT RADAR MAP ===\n");
        output.push_str(&format!(
            "Workspace: {} | Total files: {} | Symbols indexed: {}\n\n",
            workspace_name,
            total_files,
            total_symbols
        ));

        for (rel_path, mut syms) in file_map {
            output.push_str(&format!("{}:\n", rel_path));
            syms.sort_by_key(|s| s.line_number);
            for s in syms {
                output.push_str(&format!(
                    "  │ L{:<4} {:<16} {}\n",
                    s.line_number, s.symbol_kind, s.symbol_name
                ));
            }
        }

        // Token estimation heuristic (~4 chars per token for code/identifiers)
        let estimated_tokens = (output.len() / 4).max(1);

        // Optional token budgeting: if output exceeds max_tokens, truncate gracefully
        let final_content = if estimated_tokens > max_tokens && max_tokens > 100 {
            let char_limit = max_tokens * 4;
            let mut truncated = output.chars().take(char_limit).collect::<String>();
            truncated.push_str("\n... [RADAR MAP TRUNCATED TO FIT TOKEN BUDGET]\n");
            truncated
        } else {
            output
        };

        // Write to .gemini/REPO_MAP.md
        let gemini_dir = workspace_root.join(".gemini");
        let output_path = if gemini_dir.exists() || fs::create_dir_all(&gemini_dir).is_ok() {
            let target_file = gemini_dir.join("REPO_MAP.md");
            let _ = fs::write(&target_file, &final_content);
            Some(target_file)
        } else {
            None
        };

        RepoMapReport {
            total_symbols,
            total_files,
            estimated_tokens,
            map_content: final_content,
            output_file_path: output_path,
        }
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
        assert!(report.output_file_path.is_some());

        let written_file = report.output_file_path.unwrap();
        assert!(written_file.exists());

        let _ = fs::remove_dir_all(&temp_dir_path);
    }
}
