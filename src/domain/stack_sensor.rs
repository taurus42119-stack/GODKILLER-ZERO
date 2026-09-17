use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectStackContext {
    pub primary_language: String,
    pub detected_frameworks: Vec<String>,
    pub styling_paradigm: String,
    pub manifest_file_found: Option<String>,
}

impl Default for ProjectStackContext {
    fn default() -> Self {
        Self {
            primary_language: "Universal / Polyglot".to_string(),
            detected_frameworks: Vec::new(),
            styling_paradigm: "Standard CSS / Idiomatic Styling".to_string(),
            manifest_file_found: None,
        }
    }
}

pub struct UniversalStackSensor;

impl UniversalStackSensor {
    #[must_use]
    pub fn inspect_workspace(workspace_root: &Path) -> ProjectStackContext {
        let manifest_checks: [(&str, &str); 7] = [
            ("package.json", "Node.js / TypeScript Ecosystem"),
            ("Cargo.toml", "Rust Ecosystem"),
            ("pyproject.toml", "Python Ecosystem"),
            ("requirements.txt", "Python Ecosystem"),
            ("go.mod", "Go Ecosystem"),
            ("pom.xml", "Java Ecosystem"),
            ("composer.json", "PHP Ecosystem"),
        ];

        for (manifest_name, ecosystem_label) in manifest_checks {
            let manifest_path = workspace_root.join(manifest_name);
            if manifest_path.exists() {
                return Self::parse_manifest_details(
                    &manifest_path,
                    manifest_name,
                    ecosystem_label,
                );
            }
        }

        ProjectStackContext::default()
    }

    fn parse_manifest_details(
        manifest_path: &Path,
        manifest_name: &str,
        ecosystem_label: &str,
    ) -> ProjectStackContext {
        let manifest_content = fs::read_to_string(manifest_path).unwrap_or_default();
        let lower_content = manifest_content.to_lowercase();

        let mut frameworks = Vec::new();
        frameworks.push(ecosystem_label.to_string());

        let styling_paradigm = match manifest_name {
            "package.json" => Self::inspect_node_frameworks(&lower_content, &mut frameworks),
            "Cargo.toml" => Self::inspect_rust_frameworks(&lower_content, &mut frameworks),
            "pyproject.toml" | "requirements.txt" => {
                Self::inspect_python_frameworks(&lower_content, &mut frameworks)
            }
            _ => "Standard Idiomatic Styling".to_string(),
        };

        ProjectStackContext {
            primary_language: ecosystem_label.to_string(),
            detected_frameworks: frameworks,
            styling_paradigm,
            manifest_file_found: Some(manifest_name.to_string()),
        }
    }

    fn inspect_node_frameworks(content_slice: &str, frameworks: &mut Vec<String>) -> String {
        let mut styling = "Vanilla CSS / Custom Properties".to_string();

        if content_slice.contains("tailwindcss") {
            frameworks.push("TailwindCSS".to_string());
            styling =
                "TailwindCSS Utility Classes (e.g. bg-blue-600, dark:bg-slate-900)".to_string();
        } else if content_slice.contains("bootstrap") {
            frameworks.push("Bootstrap".to_string());
            styling = "Bootstrap Classes (e.g. btn-primary, bg-dark)".to_string();
        } else if content_slice.contains("styled-components") || content_slice.contains("@emotion")
        {
            frameworks.push("CSS-in-JS (Emotion / Styled)".to_string());
            styling = "CSS-in-JS Template Literals".to_string();
        }

        if content_slice.contains("next") {
            frameworks.push("Next.js App Router".to_string());
        } else if content_slice.contains("react") {
            frameworks.push("React.js".to_string());
        } else if content_slice.contains("vue") {
            frameworks.push("Vue.js".to_string());
        } else if content_slice.contains("svelte") {
            frameworks.push("Svelte".to_string());
        }

        styling
    }

    fn inspect_rust_frameworks(content_slice: &str, frameworks: &mut Vec<String>) -> String {
        if content_slice.contains("axum") {
            frameworks.push("Axum Web Framework".to_string());
        } else if content_slice.contains("actix-web") {
            frameworks.push("Actix-Web Framework".to_string());
        }

        if content_slice.contains("tokio") {
            frameworks.push("Tokio Async Runtime".to_string());
        }

        "Rust Idiomatic Type Systems & Serde".to_string()
    }

    fn inspect_python_frameworks(content_slice: &str, frameworks: &mut Vec<String>) -> String {
        if content_slice.contains("fastapi") {
            frameworks.push("FastAPI (Pydantic / Starlette)".to_string());
        } else if content_slice.contains("django") {
            frameworks.push("Django Framework".to_string());
        } else if content_slice.contains("flask") {
            frameworks.push("Flask Framework".to_string());
        }

        "PEP 8 Clean Architecture".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_inspect_workspace_defaults_on_empty() {
        let temp_directory = std::env::temp_dir().join("gk0_test_empty_workspace");
        let _ = fs::create_dir_all(&temp_directory);
        let inspection = UniversalStackSensor::inspect_workspace(&temp_directory);
        assert_eq!(inspection.primary_language, "Universal / Polyglot");
        let _ = fs::remove_dir_all(&temp_directory);
    }

    #[test]
    fn test_inspect_node_tailwind_workspace() {
        let temp_directory = std::env::temp_dir().join("gk0_test_node_tailwind");
        let _ = fs::create_dir_all(&temp_directory);
        let package_json = temp_directory.join("package.json");
        let mut file_handle = fs::File::create(&package_json).unwrap();
        writeln!(
            file_handle,
            r#"{{"dependencies": {{"next": "14.0", "react": "18.0", "tailwindcss": "3.4"}}}}"#
        )
        .unwrap();

        let inspection = UniversalStackSensor::inspect_workspace(&temp_directory);
        assert!(inspection
            .detected_frameworks
            .contains(&"TailwindCSS".to_string()));
        assert!(inspection
            .detected_frameworks
            .contains(&"Next.js App Router".to_string()));
        assert!(inspection
            .styling_paradigm
            .contains("TailwindCSS Utility Classes"));

        let _ = fs::remove_dir_all(&temp_directory);
    }
}
