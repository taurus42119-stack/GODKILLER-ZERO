use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredCoordinate {
    pub file_path: String,
    pub name_match_strength: f32,
    pub symbol_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarifierOption {
    pub action_id: String,
    pub display_label: String,
    pub description: String,
    pub prompt_patch: String,
}

pub struct AstDiscoveryEngine;

impl AstDiscoveryEngine {
    #[must_use]
    pub fn discover_coordinates_and_clarifications(
        raw_prompt: &str,
        workspace_root: &Path,
    ) -> (Vec<DiscoveredCoordinate>, Vec<ClarifierOption>) {
        Self::discover_with_tokens(raw_prompt, workspace_root, &[])
    }

    #[must_use]
    pub fn discover_with_tokens(
        raw_prompt: &str,
        workspace_root: &Path,
        candidate_tokens: &[String],
    ) -> (Vec<DiscoveredCoordinate>, Vec<ClarifierOption>) {
        let search_terms = Self::build_search_terms(raw_prompt, candidate_tokens);
        let mut candidate_files = Vec::new();
        Self::collect_workspace_source_files(workspace_root, &mut candidate_files);

        let mut discovered: Vec<DiscoveredCoordinate> = candidate_files
            .iter()
            .filter_map(|file_path| Self::score_source_file(file_path, &search_terms))
            .collect();

        discovered.sort_by(|left, right| {
            right
                .name_match_strength
                .partial_cmp(&left.name_match_strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        discovered.truncate(3);

        let clarifiers = Self::build_clarifiers(raw_prompt, discovered.first());
        (discovered, clarifiers)
    }

    fn build_search_terms(raw_prompt: &str, candidate_tokens: &[String]) -> Vec<String> {
        let mut terms: Vec<String> = raw_prompt
            .split(|character: char| {
                !character.is_alphanumeric()
                    && character != '_'
                    && character != '-'
                    && character != '.'
            })
            .filter(|token| token.len() >= 3)
            .map(str::to_lowercase)
            .collect();

        for candidate in candidate_tokens {
            let normalized = candidate.to_lowercase();
            if !terms.contains(&normalized) {
                terms.push(normalized);
            }
        }
        terms
    }

    fn score_source_file(
        file_path: &Path,
        search_terms: &[String],
    ) -> Option<DiscoveredCoordinate> {
        let path_str = file_path.to_string_lossy().replace('\\', "/");
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let lower_name = file_name.to_lowercase();
        let lower_path = path_str.to_lowercase();

        let mut match_score = 0.0f32;
        for term in search_terms {
            if lower_name.contains(term) {
                match_score += 0.6;
            } else if lower_path.contains(term) {
                match_score += 0.3;
            }
        }

        if match_score > 0.4 {
            Some(DiscoveredCoordinate {
                file_path: path_str,
                name_match_strength: match_score.min(1.0),
                symbol_hint: Some(file_name.to_string()),
            })
        } else {
            None
        }
    }

    fn build_clarifiers(
        raw_prompt: &str,
        top_match: Option<&DiscoveredCoordinate>,
    ) -> Vec<ClarifierOption> {
        let mut clarifiers = Vec::new();

        if let Some(target_match) = top_match {
            clarifiers.push(ClarifierOption {
                action_id: "PIN_DISCOVERED_FILE".into(),
                display_label: format!(
                    "Target: {}",
                    target_match.symbol_hint.as_deref().unwrap_or("file")
                ),
                description: format!("Anchor mutation strictly to {}", target_match.file_path),
                prompt_patch: format!("In file {} : {}", target_match.file_path, raw_prompt),
            });
        }

        clarifiers.push(ClarifierOption {
            action_id: "REQUEST_ASCII_BLUEPRINT".into(),
            display_label: "Request ASCII Wireframe".into(),
            description: "Force AI to output explicit ASCII component wireframe before code".into(),
            prompt_patch: format!(
                "{}\n[INVARIANT: Render explicit ASCII component wireframe before writing code]",
                raw_prompt
            ),
        });

        clarifiers.push(ClarifierOption {
            action_id: "ENFORCE_SINGLE_SCREEN".into(),
            display_label: "Restrict Function <= 70 Lines".into(),
            description: "Enforce Single-Screen function span invariant".into(),
            prompt_patch: format!(
                "{}\n[INVARIANT: Decompose logic so function spans never exceed 70 lines]",
                raw_prompt
            ),
        });

        clarifiers
    }

    fn is_ignored_ast_dir(file_name: &str) -> bool {
        file_name.starts_with('.')
            || file_name == "target"
            || file_name == "node_modules"
            || file_name == "dist"
    }

    fn process_workspace_source_entry(
        path: PathBuf,
        code_extensions: &[&str],
        accumulator: &mut Vec<PathBuf>,
    ) {
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if Self::is_ignored_ast_dir(file_name) {
            return;
        }

        if path.is_dir() {
            Self::collect_workspace_source_files(&path, accumulator);
            return;
        }

        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if code_extensions.contains(&ext) {
                accumulator.push(path);
            }
        }
    }

    fn collect_workspace_source_files(dir: &Path, accumulator: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let code_extensions = ["rs", "ts", "tsx", "js", "jsx", "py", "go", "css", "html"];

        for entry in entries.flatten() {
            Self::process_workspace_source_entry(entry.path(), &code_extensions, accumulator);
        }
    }
}
