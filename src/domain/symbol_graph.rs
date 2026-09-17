use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolNode {
    pub symbol_name: String,
    pub relative_path: String,
    pub line_number: usize,
    pub symbol_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlastRadiusAnalysis {
    pub target_symbol: String,
    pub direct_consumers: Vec<String>,
    pub blast_radius_summary: String,
}

pub struct PerProjectSymbolGraph;

impl PerProjectSymbolGraph {
    #[must_use]
    pub fn locate_symbol_and_impact(
        query_token: &str,
        workspace_root: &Path,
    ) -> Option<(SymbolNode, BlastRadiusAnalysis)> {
        let candidate_symbols = Self::harvest_symbols(workspace_root);
        let normalized_query = query_token.to_lowercase();

        let top_match = candidate_symbols.into_iter().find(|symbol| {
            let lower_name = symbol.symbol_name.to_lowercase();
            lower_name.contains(&normalized_query) || normalized_query.contains(&lower_name)
        })?;

        let consumers = Self::trace_direct_consumers(&top_match.symbol_name, workspace_root);
        let summary = if consumers.is_empty() {
            "Isolated component / zero external consumers detected".to_string()
        } else {
            format!("Direct impact on {} consumer files", consumers.len())
        };

        let analysis = BlastRadiusAnalysis {
            target_symbol: top_match.symbol_name.clone(),
            direct_consumers: consumers,
            blast_radius_summary: summary,
        };

        Some((top_match, analysis))
    }

    #[must_use]
    pub fn harvest_symbols(workspace_root: &Path) -> Vec<SymbolNode> {
        let mut source_files = Vec::new();
        Self::collect_source_paths(workspace_root, &mut source_files);

        let mut discovered_symbols = Vec::new();
        for file_path in source_files {
            Self::extract_file_symbols(&file_path, workspace_root, &mut discovered_symbols);
        }
        discovered_symbols
    }

    fn trace_direct_consumers(symbol_name: &str, workspace_root: &Path) -> Vec<String> {
        let mut source_files = Vec::new();
        Self::collect_source_paths(workspace_root, &mut source_files);

        let mut consumers = Vec::new();
        let word_boundary_regex = Regex::new(&format!(r"\b{}\b", regex::escape(symbol_name))).ok();

        for file_path in source_files {
            let Ok(content_text) = fs::read_to_string(&file_path) else {
                continue;
            };
            let matched = if let Some(ref re) = word_boundary_regex {
                re.is_match(&content_text)
            } else {
                content_text.contains(symbol_name)
            };
            if matched {
                let relative = file_path
                    .strip_prefix(workspace_root)
                    .unwrap_or(&file_path)
                    .to_string_lossy()
                    .replace('\\', "/");
                consumers.push(relative);
            }
        }
        consumers
    }

    fn extract_file_symbols(
        file_path: &Path,
        workspace_root: &Path,
        symbols_collector: &mut Vec<SymbolNode>,
    ) {
        let Ok(content_text) = fs::read_to_string(file_path) else {
            return;
        };
        let relative_path = file_path
            .strip_prefix(workspace_root)
            .unwrap_or(file_path)
            .to_string_lossy()
            .replace('\\', "/");

        for (line_index, raw_line) in content_text.lines().enumerate() {
            let line_slice = raw_line.trim();
            if let Some((name_slice, kind_slice)) = Self::parse_declaration_signature(line_slice) {
                symbols_collector.push(SymbolNode {
                    symbol_name: name_slice.to_string(),
                    relative_path: relative_path.clone(),
                    line_number: line_index + 1,
                    symbol_kind: kind_slice.to_string(),
                });
            }
        }
    }

    fn strip_declaration_modifiers(mut current: &str) -> (&str, bool) {
        let mut is_pub = false;
        loop {
            if let Some(rest) = current.strip_prefix("export default ") {
                current = rest.trim_start();
            } else if let Some(rest) = current.strip_prefix("export ") {
                current = rest.trim_start();
            } else if let Some(rest) = current.strip_prefix("pub(crate) ") {
                current = rest.trim_start();
                is_pub = true;
            } else if let Some(rest) = current.strip_prefix("pub(super) ") {
                current = rest.trim_start();
                is_pub = true;
            } else if let Some(rest) = current.strip_prefix("pub ") {
                current = rest.trim_start();
                is_pub = true;
            } else if let Some(rest) = current.strip_prefix("async ") {
                current = rest.trim_start();
            } else if let Some(rest) = current.strip_prefix("unsafe ") {
                current = rest.trim_start();
            } else {
                break;
            }
        }
        (current, is_pub)
    }

    fn is_comment_line(trimmed: &str) -> bool {
        trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
    }

    fn match_pattern_prefix<'a>(
        current: &'a str,
        prefix: &str,
        kind: &'a str,
    ) -> Option<(&'a str, &'a str)> {
        let remainder = current.strip_prefix(prefix)?;
        let identifier = remainder
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .unwrap_or("")
            .trim();
        if identifier.len() >= 3 && !identifier.starts_with('_') {
            Some((identifier, kind))
        } else {
            None
        }
    }

    fn parse_declaration_signature(line_slice: &str) -> Option<(&str, &str)> {
        let trimmed = line_slice.trim();
        if Self::is_comment_line(trimmed) {
            return None;
        }

        let (current, is_pub) = Self::strip_declaration_modifiers(trimmed);

        let patterns: [(&str, &str); 10] = [
            ("function ", "Function"),
            (
                "fn ",
                if is_pub {
                    "Public Function"
                } else {
                    "Function"
                },
            ),
            ("def ", "Function"),
            ("class ", if is_pub { "Public Class" } else { "Class" }),
            ("struct ", if is_pub { "Public Struct" } else { "Struct" }),
            ("interface ", "Interface"),
            ("type ", "Type"),
            ("trait ", "Trait"),
            ("enum ", "Enum"),
            ("const ", "Constant/Component"),
        ];

        for (prefix, kind) in patterns {
            if let Some(matched) = Self::match_pattern_prefix(current, prefix, kind) {
                return Some(matched);
            }
        }
        None
    }

    fn is_ignored_source_entry(name: &str) -> bool {
        name.starts_with('.')
            || name == "target"
            || name == "node_modules"
            || name == "dist"
            || name == "bin"
            || name == "obj"
    }

    fn process_source_path_entry(
        entry_path: PathBuf,
        valid_extensions: &[&str],
        paths_collector: &mut Vec<PathBuf>,
    ) {
        let entry_name = entry_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if Self::is_ignored_source_entry(entry_name) {
            return;
        }
        if fs::symlink_metadata(&entry_path)
            .map(|m| m.is_symlink())
            .unwrap_or(false)
        {
            return;
        }

        if entry_path.is_dir() {
            Self::collect_source_paths(&entry_path, paths_collector);
        } else if entry_path.is_file() {
            let extension_slice = entry_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            if valid_extensions.contains(&extension_slice) {
                paths_collector.push(entry_path);
            }
        }
    }

    fn collect_source_paths(current_dir: &Path, paths_collector: &mut Vec<PathBuf>) {
        let Ok(directory_entries) = fs::read_dir(current_dir) else {
            return;
        };
        let valid_extensions = [
            "tsx", "ts", "jsx", "js", "rs", "py", "go", "vue", "svelte", "cs",
        ];

        for entry in directory_entries.flatten() {
            Self::process_source_path_entry(entry.path(), &valid_extensions, paths_collector);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_harvest_and_trace_blast_radius() {
        let temp_dir = std::env::temp_dir().join("gk0_test_blast_radius");
        let _ = fs::create_dir_all(&temp_dir);

        let nav_file = temp_dir.join("Navigation.tsx");
        let mut nav_handle = fs::File::create(&nav_file).unwrap();
        writeln!(
            nav_handle,
            "export function SubmitButton() {{ return <button>Submit</button>; }}"
        )
        .unwrap();

        let header_file = temp_dir.join("Header.tsx");
        let mut header_handle = fs::File::create(&header_file).unwrap();
        writeln!(
            header_handle,
            "import {{ SubmitButton }} from './Navigation';\nexport function Header() {{ return <SubmitButton />; }}"
        )
        .unwrap();

        let query_outcome =
            PerProjectSymbolGraph::locate_symbol_and_impact("SubmitButton", &temp_dir);
        assert!(query_outcome.is_some());
        let (node, blast) = query_outcome.unwrap();
        assert_eq!(node.symbol_name, "SubmitButton");
        assert_eq!(node.relative_path, "Navigation.tsx");
        assert!(blast.direct_consumers.contains(&"Header.tsx".to_string()));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_parse_advanced_signatures_and_ignore_comments() {
        // Comments should be ignored
        assert_eq!(
            PerProjectSymbolGraph::parse_declaration_signature("// pub fn old_code()"),
            None
        );
        assert_eq!(
            PerProjectSymbolGraph::parse_declaration_signature("/* export function fake() */"),
            None
        );

        // Rust pub(crate) async fn
        let parsed_crate = PerProjectSymbolGraph::parse_declaration_signature(
            "pub(crate) async fn dispatch_signal()",
        );
        assert_eq!(parsed_crate, Some(("dispatch_signal", "Public Function")));

        // TypeScript interface & type
        let parsed_iface = PerProjectSymbolGraph::parse_declaration_signature(
            "export interface UserProfilePayload {",
        );
        assert_eq!(parsed_iface, Some(("UserProfilePayload", "Interface")));

        let parsed_type = PerProjectSymbolGraph::parse_declaration_signature(
            "export type SecurityRole = string;",
        );
        assert_eq!(parsed_type, Some(("SecurityRole", "Type")));
    }
}
