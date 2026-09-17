use std::path::Path;
use tree_sitter::{Language, Node, Parser};

#[derive(Debug, Clone)]
pub struct AstFunctionMetrics {
    pub function_name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub span_lines: usize,
    pub cognitive_complexity: usize,
    pub is_exempt: bool,
}

#[derive(Debug, Clone)]
pub struct AstAnalysisReport {
    pub detected_language: &'static str,
    pub functions: Vec<AstFunctionMetrics>,
}

pub struct AstEngine;

impl AstEngine {
    #[must_use]
    pub fn is_supported_file(file_path: &Path) -> bool {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        matches!(
            extension,
            "rs" | "cs" | "ts" | "tsx" | "js" | "jsx" | "py"
        )
    }

    #[must_use]
    pub fn parse_file(file_path: &Path, source_code: &str) -> Option<AstAnalysisReport> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let (language_name, ts_language) = Self::resolve_language(extension)?;
        let mut parser = Parser::new();
        if parser.set_language(&ts_language).is_err() {
            return None;
        }

        let parsed_tree = parser.parse(source_code, None)?;
        let path_str = file_path.to_string_lossy().replace('\\', "/");
        let is_test_context = Self::is_test_file_path(file_path);
        let is_structural_exempt = is_test_context
            || path_str.ends_with("Form.cs")
            || path_str.starts_with("gui-")
            || path_str.contains("/gui-")
            || path_str.starts_with("src-ui/")
            || path_str.contains("/src-ui/")
            || path_str.contains("/ui/")
            || path_str.contains("/views/")
            || path_str.contains("/components/");

        let mut discovered_functions = Vec::new();
        Self::traverse_for_functions(
            parsed_tree.root_node(),
            source_code,
            language_name,
            is_structural_exempt,
            &mut discovered_functions,
        );

        Some(AstAnalysisReport {
            detected_language: language_name,
            functions: discovered_functions,
        })
    }

    fn resolve_language(extension: &str) -> Option<(&'static str, Language)> {
        match extension {
            "rs" => Some(("rust", tree_sitter_rust::LANGUAGE.into())),
            "cs" => Some(("c_sharp", tree_sitter_c_sharp::LANGUAGE.into())),
            "ts" | "js" => Some((
                "typescript",
                tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            )),
            "tsx" | "jsx" => Some(("tsx", tree_sitter_typescript::LANGUAGE_TSX.into())),
            "py" => Some(("python", tree_sitter_python::LANGUAGE.into())),
            _ => None,
        }
    }

    fn is_test_file_path(file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().replace('\\', "/");
        path_str.contains("/tests/")
            || path_str.starts_with("tests/")
            || path_str.contains("/test/")
            || path_str.contains("/fixtures/")
            || path_str.ends_with("_test.rs")
            || path_str.ends_with(".test.ts")
            || path_str.ends_with(".spec.ts")
            || path_str.ends_with(".test.js")
            || path_str.ends_with(".spec.js")
            || path_str.contains("/test_")
    }

    fn is_function_node(node_kind: &str, language: &str) -> bool {
        match language {
            "rust" => node_kind == "function_item",
            "c_sharp" => matches!(
                node_kind,
                "method_declaration"
                    | "local_function_statement"
                    | "constructor_declaration"
            ),
            "typescript" | "tsx" => matches!(
                node_kind,
                "function_declaration"
                    | "method_definition"
                    | "arrow_function"
                    | "function_expression"
            ),
            "python" => node_kind == "function_definition",
            _ => false,
        }
    }

    fn traverse_for_functions(
        current_node: Node,
        source_code: &str,
        language: &str,
        is_structural_exempt: bool,
        target_list: &mut Vec<AstFunctionMetrics>,
    ) {
        if Self::is_function_node(current_node.kind(), language) {
            let metric_entry = Self::measure_function(
                current_node,
                source_code,
                language,
                is_structural_exempt,
            );
            target_list.push(metric_entry);
        }

        let mut child_cursor = current_node.walk();
        for child_element in current_node.children(&mut child_cursor) {
            Self::traverse_for_functions(
                child_element,
                source_code,
                language,
                is_structural_exempt,
                target_list,
            );
        }
    }

    fn measure_function(
        function_node: Node,
        source_code: &str,
        language: &str,
        is_structural_exempt: bool,
    ) -> AstFunctionMetrics {
        let start_pos = function_node.start_position();
        let end_pos = function_node.end_position();
        let line_span = end_pos.row - start_pos.row + 1;
        let start_row_indexed = start_pos.row + 1;
        let end_row_indexed = end_pos.row + 1;

        let symbol_identifier = Self::extract_function_name(function_node, source_code)
            .unwrap_or_else(|| format!("fn@L{}", start_row_indexed));

        let has_declarative_jsx = (language == "tsx" || language == "typescript")
            && Self::has_descendant_kind(function_node, "jsx_element");
        let is_exempt = is_structural_exempt
            || has_declarative_jsx
            || symbol_identifier == "InitializeComponent"
            || symbol_identifier.ends_with("Form");

        let calculated_complexity =
            Self::calculate_cognitive_complexity(function_node, 0);

        AstFunctionMetrics {
            function_name: symbol_identifier,
            start_line: start_row_indexed,
            end_line: end_row_indexed,
            span_lines: line_span,
            cognitive_complexity: calculated_complexity,
            is_exempt,
        }
    }

    fn extract_function_name(node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "name" || child.kind() == "identifier" {
                let byte_slice = &source_code.as_bytes()[child.start_byte()..child.end_byte()];
                if let Ok(extracted) = std::str::from_utf8(byte_slice) {
                    return Some(extracted.trim().to_string());
                }
            }
        }
        None
    }

    fn has_descendant_kind(parent_node: Node, target_kind: &str) -> bool {
        if parent_node.kind() == target_kind {
            return true;
        }
        let mut cursor = parent_node.walk();
        for child in parent_node.children(&mut cursor) {
            if Self::has_descendant_kind(child, target_kind) {
                return true;
            }
        }
        false
    }

    fn calculate_cognitive_complexity(node: Node, current_nesting: usize) -> usize {
        let mut complexity_score = 0;
        let is_control_flow = Self::is_cognitive_increment_node(node.kind());
        let is_else_if_branch = is_control_flow && Self::is_else_if(node);

        let next_nesting = if is_else_if_branch {
            complexity_score += 1;
            current_nesting
        } else if is_control_flow {
            complexity_score += 1 + current_nesting;
            current_nesting + 1
        } else if Self::is_nesting_barrier(node.kind()) {
            current_nesting + 1
        } else {
            current_nesting
        };

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "function_item"
                || child.kind() == "method_declaration"
                || child.kind() == "function_definition"
            {
                continue;
            }
            complexity_score += Self::calculate_cognitive_complexity(child, next_nesting);
        }

        complexity_score
    }

    fn is_else_if(node: Node) -> bool {
        if let Some(parent) = node.parent() {
            let parent_kind = parent.kind();
            if parent_kind == "else_clause" || parent_kind == "else" {
                return true;
            }
            if (parent_kind == "if_statement" || parent_kind == "if_expression")
                && parent.child_by_field_name("alternative").map(|n| n.id()) == Some(node.id())
            {
                return true;
            }
        }
        false
    }

    fn is_cognitive_increment_node(node_kind: &str) -> bool {
        matches!(
            node_kind,
            "if_statement"
                | "if_expression"
                | "while_statement"
                | "while_expression"
                | "for_statement"
                | "for_expression"
                | "for_in_statement"
                | "catch_clause"
                | "conditional_expression"
        )
    }

    fn is_nesting_barrier(node_kind: &str) -> bool {
        matches!(
            node_kind,
            "match_expression"
                | "switch_statement"
                | "switch_expression"
                | "try_statement"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ast_rust_function_span() {
        let rust_code = r#"
fn short_function() {
    let mut total = 0;
    for number in 0..10 {
        if number % 2 == 0 {
            total += number;
        }
    }
}
"#;
        let report = AstEngine::parse_file(&PathBuf::from("src/test.rs"), rust_code)
            .expect("Valid rust parse");
        assert_eq!(report.functions.len(), 1);
        let metric = &report.functions[0];
        assert_eq!(metric.function_name, "short_function");
        assert_eq!(metric.span_lines, 8);
        assert!(!metric.is_exempt);
        assert!(metric.cognitive_complexity >= 2);
    }

    #[test]
    fn test_ast_csharp_cognitive_complexity() {
        let csharp_code = r#"
public class Processor {
    public void ProcessAccounts() {
        if (true) {
            if (false) {
                while (true) {}
            }
        }
    }
}
"#;
        let report = AstEngine::parse_file(&PathBuf::from("Services/Processor.cs"), csharp_code)
            .expect("Valid csharp parse");
        assert_eq!(report.functions.len(), 1);
        let metric = &report.functions[0];
        assert_eq!(metric.function_name, "ProcessAccounts");
        assert!(metric.cognitive_complexity >= 4);
    }

    #[test]
    fn test_ast_tsx_declarative_immunity() {
        let tsx_code = r#"
export const DashboardView = () => {
    return (
        <div className="flex">
            <span>Overview</span>
        </div>
    );
};
"#;
        let report = AstEngine::parse_file(&PathBuf::from("components/Dashboard.tsx"), tsx_code)
            .expect("Valid tsx parse");
        assert_eq!(report.functions.len(), 1);
        let metric = &report.functions[0];
        assert!(metric.is_exempt, "JSX components must have structural immunity");
    }
}
