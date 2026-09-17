use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use super::ast_engine::AstEngine;

static BANNED_IDENTIFIER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:const|let|var)\s+(?:mut\s+)?(data|res|req|item|val|temp|obj|info|payload|result)\b|",
        r"(?i)\bfunction\s+(handleData|processData|doAction)\b|",
        r"(?i)(?:^|[^.\w$])(data|res|req|item|val|temp|obj|info|payload|result)(?:\s*,\s*\w+)*\s*(?::=|=)\s*[^=]|",
        r"(?i)\((?:\s*|\w+\s*,\s*)(payload|val|temp)\s*(?::|,|\))"
    ))
    .expect("Valid banned identifier regex")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatekeeperViolation {
    pub file_path: String,
    pub line_number: usize,
    pub rule_identifier: String,
    pub description: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatekeeperAuditResult {
    pub passed: bool,
    pub total_files_scanned: usize,
    pub violations: Vec<GatekeeperViolation>,
    pub summary_message: String,
}

pub struct GatekeeperScanner;

impl GatekeeperScanner {
    #[must_use]
    pub fn scan_path(
        target_directory: &Path,
        max_span_threshold: usize,
        max_complexity_threshold: usize,
    ) -> GatekeeperAuditResult {
        let mut violations = Vec::new();
        let mut total_files = 0;

        let mut process_file = |file_path: &Path| {
            if Self::audit_single_file(
                file_path,
                max_span_threshold,
                max_complexity_threshold,
                &mut violations,
            ) {
                total_files += 1;
            }
        };

        if target_directory.is_file() {
            process_file(target_directory);
        } else {
            Self::walk_directory(target_directory, &mut process_file);
        }

        let passed = violations.is_empty();
        let summary_message = if passed {
            format!(
                "✓ [GATEKEEPER PASS] Scanned {} files. Zero invariant violations detected.",
                total_files
            )
        } else {
            format!(
                "❌ [GATEKEEPER REJECT] {} violations detected across {} files.",
                violations.len(),
                total_files
            )
        };

        GatekeeperAuditResult {
            passed,
            total_files_scanned: total_files,
            violations,
            summary_message,
        }
    }

    fn audit_single_file(
        file_path: &Path,
        max_span_threshold: usize,
        max_complexity_threshold: usize,
        violations: &mut Vec<GatekeeperViolation>,
    ) -> bool {
        let code_extensions = ["rs", "ts", "tsx", "js", "jsx", "py", "go", "cs", "java"];
        let ext = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !code_extensions.contains(&ext) {
            return false;
        }

        let path_string = file_path.to_string_lossy().replace('\\', "/");
        Self::check_junk_drawer_rule(&path_string, violations);

        if let Ok(file_content) = fs::read_to_string(file_path) {
            let lines: Vec<&str> = file_content.lines().collect();
            Self::check_generic_identifiers_rule(&lines, &path_string, violations);
            Self::check_lazy_stubs_and_error_hygiene_rule(&lines, &path_string, ext, violations);
            let span_violations = Self::check_function_spans(
                &lines,
                &path_string,
                max_span_threshold,
                max_complexity_threshold,
            );
            violations.extend(span_violations);
        }
        true
    }

    fn check_junk_drawer_rule(path_string: &str, violations: &mut Vec<GatekeeperViolation>) {
        if path_string.contains("/utils/")
            || path_string.contains("/helpers/")
            || path_string.contains("/common/")
            || path_string.starts_with("utils/")
            || path_string.starts_with("helpers/")
            || path_string.starts_with("common/")
        {
            violations.push(GatekeeperViolation {
                file_path: path_string.to_string(),
                line_number: 1,
                rule_identifier: "RULE_3_BAN_JUNK_DRAWERS".into(),
                description:
                    "Forbidden junk drawer directory (prohibited: utils/, helpers/, common/)"
                        .into(),
                snippet: path_string.to_string(),
            });
        }
    }

    fn check_generic_identifiers_rule(
        lines: &[&str],
        path_string: &str,
        violations: &mut Vec<GatekeeperViolation>,
    ) {
        let is_test_file = path_string.contains("/tests/")
            || path_string.starts_with("tests/")
            || path_string.starts_with("./tests/")
            || path_string.contains("/test/")
            || path_string.contains("/fixtures/")
            || path_string.ends_with("_test.rs")
            || path_string.ends_with(".test.ts")
            || path_string.ends_with(".spec.ts")
            || path_string.ends_with(".test.js")
            || path_string.ends_with(".spec.js")
            || path_string.contains("/test_");

        if is_test_file {
            return;
        }

        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//")
                || trimmed.starts_with('#')
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
            {
                continue;
            }

            let stripped_line = Self::strip_comments_and_strings(trimmed);
            if let Some(m) = BANNED_IDENTIFIER_PATTERN.find(&stripped_line) {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: index + 1,
                    rule_identifier: "RULE_2_BAN_GENERIC_IDENTIFIERS".into(),
                    description: format!("Generic identifier detected: '{}'", m.as_str()),
                    snippet: trimmed.to_string(),
                });
            }
        }
    }

    fn check_comment_stubs(
        trimmed: &str,
        path_string: &str,
        line_num: usize,
        violations: &mut Vec<GatekeeperViolation>,
    ) {
        let is_explicit_stub = trimmed.contains("TODO")
            || trimmed.contains("todo:")
            || trimmed.starts_with("// todo ")
            || trimmed.starts_with("# todo ");
        if is_explicit_stub
            && !trimmed.contains("RULE_12")
            && !trimmed.contains("FORBIDDEN")
            && !trimmed.contains("banned")
            && !path_string.contains("gatekeeper.rs")
        {
            violations.push(GatekeeperViolation {
                file_path: path_string.to_string(),
                line_number: line_num,
                rule_identifier: "RULE_12_ANTI_LAZY_STUBS".into(),
                description: "Forbidden lazy stub comment detected (TODO / unimplemented). Complete production logic required.".into(),
                snippet: trimmed.to_string(),
            });
        }
    }

    fn is_empty_catch_block(stripped: &str, index: usize, lines: &[&str]) -> bool {
        let is_catch_start = stripped.starts_with("catch")
            || stripped.contains(" catch ")
            || stripped.contains("catch(")
            || stripped.contains("catch {");

        if !is_catch_start {
            return false;
        }

        if stripped.contains("{}") || stripped.contains("{ }") {
            return true;
        }

        let mut next_idx = index + 1;
        while next_idx < lines.len() && lines[next_idx].trim().is_empty() {
            next_idx += 1;
        }

        if next_idx < lines.len() {
            let next_line = lines[next_idx].trim();
            if next_line == "{}" || next_line == "{ }" {
                return true;
            }
            if next_line == "{" {
                let mut after_open = next_idx + 1;
                while after_open < lines.len() && lines[after_open].trim().is_empty() {
                    after_open += 1;
                }
                if after_open < lines.len() && lines[after_open].trim() == "}" {
                    return true;
                }
            }
        }

        false
    }

    fn check_python_and_js_stubs(
        stripped: &str,
        trimmed: &str,
        path_string: &str,
        ext: &str,
        line_num: usize,
        index: usize,
        lines: &[&str],
        violations: &mut Vec<GatekeeperViolation>,
    ) {
        if ext == "py" {
            if stripped == "except:" || stripped.starts_with("except ") {
                if stripped.ends_with("pass") || (index + 1 < lines.len() && lines[index + 1].trim() == "pass") {
                    violations.push(GatekeeperViolation {
                        file_path: path_string.to_string(),
                        line_number: line_num,
                        rule_identifier: "RULE_12_NO_SILENT_CATCH".into(),
                        description: "Forbidden silent error suppression ('except: pass'). Handle or log errors explicitly.".into(),
                        snippet: trimmed.to_string(),
                    });
                }
            }
            if stripped.contains("raise NotImplementedError") {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: line_num,
                    rule_identifier: "RULE_12_ANTI_LAZY_STUBS".into(),
                    description: "Forbidden NotImplemented stub in production logic.".into(),
                    snippet: trimmed.to_string(),
                });
            }
        }

        if matches!(ext, "ts" | "tsx" | "js" | "jsx") && Self::is_empty_catch_block(stripped, index, lines) {
            violations.push(GatekeeperViolation {
                file_path: path_string.to_string(),
                line_number: line_num,
                rule_identifier: "RULE_12_NO_SILENT_CATCH".into(),
                description: "Forbidden silent catch block ('catch {}'). Handle or rethrow errors explicitly.".into(),
                snippet: trimmed.to_string(),
            });
        }
    }

    fn check_compiled_lang_hygiene(
        stripped: &str,
        trimmed: &str,
        path_string: &str,
        ext: &str,
        line_num: usize,
        index: usize,
        lines: &[&str],
        in_test_module: bool,
        is_test_file: bool,
        violations: &mut Vec<GatekeeperViolation>,
    ) {
        if matches!(ext, "cs" | "java") {
            if stripped.contains("NotImplementedException") {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: line_num,
                    rule_identifier: "RULE_12_ANTI_LAZY_STUBS".into(),
                    description: "Forbidden NotImplementedException stub in production code.".into(),
                    snippet: trimmed.to_string(),
                });
            }
            if Self::is_empty_catch_block(stripped, index, lines) {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: line_num,
                    rule_identifier: "RULE_12_NO_SILENT_CATCH".into(),
                    description: "Forbidden empty catch block in C#/Java.".into(),
                    snippet: trimmed.to_string(),
                });
            }
        }

        if ext == "rs" && !is_test_file && !in_test_module {
            if (stripped.contains(".unwrap()") || stripped.contains(".expect("))
                && !path_string.contains("gatekeeper.rs")
                && !path_string.contains("test")
            {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: line_num,
                    rule_identifier: "RULE_12_EXHAUSTIVE_ERROR_HANDLING".into(),
                    description: "Forbidden unchecked unwrap()/expect() in production Rust code. Use Result<T, E> and '?' operator.".into(),
                    snippet: trimmed.to_string(),
                });
            }
        }
    }

    fn check_lazy_stubs_and_error_hygiene_rule(
        lines: &[&str],
        path_string: &str,
        ext: &str,
        violations: &mut Vec<GatekeeperViolation>,
    ) {
        let is_test_file = path_string.contains("/tests/")
            || path_string.starts_with("tests/")
            || path_string.starts_with("./tests/")
            || path_string.contains("/test/")
            || path_string.contains("/fixtures/")
            || path_string.ends_with("_test.rs")
            || path_string.ends_with(".test.ts")
            || path_string.ends_with(".spec.ts")
            || path_string.ends_with(".test.js")
            || path_string.ends_with(".spec.js")
            || path_string.contains("/test_");

        let mut in_test_module = false;

        for (index, line) in lines.iter().enumerate() {
            let line_num = index + 1;
            let trimmed = line.trim();

            if ext == "rs" && (trimmed.starts_with("#[cfg(test)]") || trimmed.starts_with("mod test") || trimmed.starts_with("mod tests")) {
                in_test_module = true;
            }

            if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                Self::check_comment_stubs(trimmed, path_string, line_num, violations);
                continue;
            }

            let stripped = Self::strip_comments_and_strings(trimmed);

            if (stripped.contains("todo!()") || stripped.contains("unimplemented!()"))
                && !stripped.contains("RULE_12")
                && !stripped.contains("FORBIDDEN")
            {
                violations.push(GatekeeperViolation {
                    file_path: path_string.to_string(),
                    line_number: line_num,
                    rule_identifier: "RULE_12_ANTI_LAZY_STUBS".into(),
                    description: "Forbidden lazy stub macro in active code (todo! / unimplemented!).".into(),
                    snippet: trimmed.to_string(),
                });
            }

            Self::check_python_and_js_stubs(
                &stripped, trimmed, path_string, ext, line_num, index, lines, violations,
            );
            Self::check_compiled_lang_hygiene(
                &stripped, trimmed, path_string, ext, line_num, index, lines, in_test_module, is_test_file, violations,
            );
        }
    }

    fn push_span_violation(
        violations: &mut Vec<GatekeeperViolation>,
        file_path: &str,
        start_line: usize,
        span: usize,
        max_span: usize,
        snippet: &str,
    ) {
        if span > max_span {
            violations.push(GatekeeperViolation {
                file_path: file_path.to_string(),
                line_number: start_line,
                rule_identifier: "RULE_4_MAX_FUNCTION_SPAN".into(),
                description: format!(
                    "Function exceeds span budget: {} lines (max: {})",
                    span, max_span
                ),
                snippet: snippet.to_string(),
            });
        }
    }

    fn check_function_spans(
        lines: &[&str],
        file_path: &str,
        max_span: usize,
        max_complexity: usize,
    ) -> Vec<GatekeeperViolation> {
        let path = Path::new(file_path);
        let joined_content = lines.join("\n");
        if AstEngine::is_supported_file(path) {
            if let Some(report) = AstEngine::parse_file(path, &joined_content) {
                let mut violations = Vec::new();
                for function in report.functions {
                    if function.is_exempt {
                        continue;
                    }
                    if function.span_lines > max_span {
                        violations.push(GatekeeperViolation {
                            file_path: file_path.to_string(),
                            line_number: function.start_line,
                            rule_identifier: "RULE_4_MAX_FUNCTION_SPAN".into(),
                            description: format!(
                                "Function exceeds span budget: {} lines (max: {})",
                                function.span_lines, max_span
                            ),
                            snippet: function.function_name.clone(),
                        });
                    }
                    if function.cognitive_complexity > max_complexity {
                        violations.push(GatekeeperViolation {
                            file_path: file_path.to_string(),
                            line_number: function.start_line,
                            rule_identifier: "RULE_3_MAX_COGNITIVE_COMPLEXITY".into(),
                            description: format!(
                                "Function exceeds cognitive complexity budget: {} (max: {})",
                                function.cognitive_complexity, max_complexity
                            ),
                            snippet: function.function_name.clone(),
                        });
                    }
                }
                return violations;
            }
        }

        if file_path.ends_with(".py") {
            return Self::check_python_function_spans(lines, file_path, max_span);
        }
        Self::check_brace_function_spans(lines, file_path, max_span)
    }

    fn check_python_function_spans(
        lines: &[&str],
        file_path: &str,
        max_span: usize,
    ) -> Vec<GatekeeperViolation> {
        let mut violations = Vec::new();
        let fn_start_pattern =
            Regex::new(r"^(?:\s*)(?:async\s+)?def\s+([a-zA-Z0-9_]+)\s*\(").expect("Valid py def regex");

        let mut current_fn: Option<(usize, usize, String)> = None;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let current_indent = line.len() - line.trim_start().len();

            if let Some(captures) = fn_start_pattern.captures(line) {
                if let Some((start_line, _, ref prev_fn)) = current_fn {
                    Self::push_span_violation(
                        &mut violations,
                        file_path,
                        start_line,
                        line_num - start_line,
                        max_span,
                        prev_fn,
                    );
                }
                let fn_name = captures.get(1).map(|m| m.as_str()).unwrap_or("fn");
                current_fn = Some((line_num, current_indent, fn_name.to_string()));
                continue;
            }

            if let Some((start_line, base_indent, ref fn_name)) = current_fn {
                if current_indent <= base_indent {
                    Self::push_span_violation(
                        &mut violations,
                        file_path,
                        start_line,
                        line_num - start_line,
                        max_span,
                        fn_name,
                    );
                    current_fn = None;
                }
            }
        }

        if let Some((start_line, _, ref fn_name)) = current_fn {
            Self::push_span_violation(
                &mut violations,
                file_path,
                start_line,
                lines.len() - start_line + 1,
                max_span,
                fn_name,
            );
        }

        violations
    }

    fn detect_brace_fn_start(
        trimmed: &str,
        sanitized: &str,
        line_num: usize,
        idx: usize,
        current_fn_start: &mut Option<(usize, String)>,
        pending_fn: &mut Option<(usize, String)>,
        brace_depth: &mut i32,
        pattern: &Regex,
    ) {
        if let Some((start_line, name)) = pending_fn.take() {
            if sanitized.contains('{') {
                *current_fn_start = Some((start_line, name));
                *brace_depth = 0;
            } else if idx - start_line <= 2 {
                *pending_fn = Some((start_line, name));
            }
        } else if pattern.is_match(trimmed) {
            if sanitized.contains('{') {
                *current_fn_start = Some((line_num, trimmed.to_string()));
                *brace_depth = 0;
            } else {
                *pending_fn = Some((line_num, trimmed.to_string()));
            }
        }
    }

    fn update_brace_depth(
        sanitized: &str,
        current_fn_start: &mut Option<(usize, String)>,
        brace_depth: &mut i32,
        violations: &mut Vec<GatekeeperViolation>,
        file_path: &str,
        line_num: usize,
        max_span: usize,
    ) {
        if let Some((start_line, ref fn_name)) = current_fn_start {
            for ch in sanitized.chars() {
                if ch == '{' {
                    *brace_depth += 1;
                } else if ch == '}' {
                    *brace_depth -= 1;
                    if *brace_depth <= 0 {
                        Self::push_span_violation(
                            violations,
                            file_path,
                            *start_line,
                            line_num - *start_line + 1,
                            max_span,
                            fn_name,
                        );
                        *current_fn_start = None;
                        *brace_depth = 0;
                        break;
                    }
                }
            }
        }
    }

    fn check_brace_function_spans(
        lines: &[&str],
        file_path: &str,
        max_span: usize,
    ) -> Vec<GatekeeperViolation> {
        let mut violations = Vec::new();
        let mut current_fn_start: Option<(usize, String)> = None;
        let mut pending_fn: Option<(usize, String)> = None;
        let mut brace_depth: i32 = 0;

        let fn_start_pattern =
            Regex::new(r"(?i)\b(?:fn|function|const\s+\w+\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\b")
                .expect("Valid fn start regex");
        let mut in_raw_string = false;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if in_raw_string {
                if trimmed.contains("\"#") {
                    in_raw_string = false;
                }
                continue;
            }

            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                continue;
            }

            if (trimmed.contains("r#\"") || trimmed.contains("r##\"")) && !trimmed.contains("\"#") {
                in_raw_string = true;
            }

            let sanitized = Self::strip_comments_and_strings(line);

            if current_fn_start.is_none() {
                Self::detect_brace_fn_start(
                    trimmed,
                    &sanitized,
                    line_num,
                    idx,
                    &mut current_fn_start,
                    &mut pending_fn,
                    &mut brace_depth,
                    &fn_start_pattern,
                );
                if current_fn_start.is_none() && pending_fn.is_some() {
                    continue;
                }
            }

            Self::update_brace_depth(
                &sanitized,
                &mut current_fn_start,
                &mut brace_depth,
                &mut violations,
                file_path,
                line_num,
                max_span,
            );
        }

        violations
    }

    fn strip_comments_and_strings(line: &str) -> String {
        let mut char_accumulator = String::with_capacity(line.len());
        let mut in_quote: Option<char> = None;
        let mut prev_char = ' ';
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if in_quote.is_none() && ch == '/' && chars.peek() == Some(&'/') {
                break;
            }

            if let Some(q) = in_quote {
                if ch == q && prev_char != '\\' {
                    in_quote = None;
                }
            } else if (ch == '"' || ch == '\'' || ch == '`') && prev_char != '\\' {
                in_quote = Some(ch);
            } else {
                char_accumulator.push(ch);
            }
            prev_char = ch;
        }

        char_accumulator
    }

    fn walk_directory(dir: &Path, callback: &mut dyn FnMut(&Path)) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

            if file_name.starts_with('.')
                || file_name == "target"
                || file_name == "node_modules"
                || file_name == "dist"
            {
                continue;
            }

            if path.is_dir() {
                Self::walk_directory(&path, callback);
            } else if path.is_file() {
                callback(&path);
            }
        }
    }

    pub fn install_git_pre_commit_hook(workspace_dir: &Path) -> Result<PathBuf, String> {
        let git_dir = workspace_dir.join(".git");
        if !git_dir.exists() {
            return Err(format!(
                "Target path '{}' is not a Git repository (no .git folder found). Please run inside your git repository or pass target path.",
                workspace_dir.display()
            ));
        }

        let git_hooks_dir = git_dir.join("hooks");
        if !git_hooks_dir.exists() {
            fs::create_dir_all(&git_hooks_dir)
                .map_err(|e| format!("Failed to create .git/hooks directory: {}", e))?;
        }

        let hook_script = r#"#!/bin/sh
# GODKILLER ZERO : Cognitive Pre-flight Disk Gatekeeper Hook
echo "🛡️  [GODKILLER ZERO] Inspecting staged codebase for architectural invariants..."

if command -v godkiller-zero >/dev/null 2>&1; then
    EXE_CMD="godkiller-zero"
elif [ -f "./godkiller-zero.exe" ]; then
    EXE_CMD="./godkiller-zero.exe"
elif [ -f "./target/release/godkiller-zero.exe" ]; then
    EXE_CMD="./target/release/godkiller-zero.exe"
else
    EXE_CMD=""
fi

if [ -n "$EXE_CMD" ]; then
    STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null | grep -E '\.(rs|ts|tsx|js|jsx|py|go|cs)$')
    if [ -n "$STAGED_FILES" ]; then
        for FILE in $STAGED_FILES; do
            if [ -f "$FILE" ]; then
                "$EXE_CMD" --gate "$FILE"
                GATE_EXIT=$?
                if [ $GATE_EXIT -ne 0 ]; then
                    echo "❌ [GODKILLER ZERO] Pre-commit gate failed on $FILE! Fix architectural violations before committing."
                    exit 1
                fi
            fi
        done
    fi
else
    echo "⚠️  [GODKILLER ZERO] Binary 'godkiller-zero' not found in PATH or project root, skipping pre-commit gate."
fi

exit 0
"#;

        let hook_file = git_hooks_dir.join("pre-commit");
        fs::write(&hook_file, hook_script)
            .map_err(|e| format!("Failed to write pre-commit hook file: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&hook_file) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&hook_file, perms);
            }
        }

        Ok(hook_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_function_span_tracking() {
        let py_code = "def short_fn():\n    x = 1\n    return x\n";
        let lines: Vec<&str> = py_code.lines().collect();
        let violations = GatekeeperScanner::check_function_spans(&lines, "test.py", 5, 7);
        assert!(violations.is_empty());

        let long_py_code = "def long_fn():\n".to_string() + &"    pass\n".repeat(20);
        let long_lines: Vec<&str> = long_py_code.lines().collect();
        let long_violations = GatekeeperScanner::check_function_spans(&long_lines, "test.py", 10, 7);
        assert_eq!(long_violations.len(), 1);
        assert_eq!(long_violations[0].rule_identifier, "RULE_4_MAX_FUNCTION_SPAN");
    }

    #[test]
    fn test_brace_string_immunity() {
        let js_code = r#"function parseTokens() {
    const template = "{ignore_me}";
    const nested = "{{also_ignore}}";
    return template;
}"#;
        let lines: Vec<&str> = js_code.lines().collect();
        let violations = GatekeeperScanner::check_function_spans(&lines, "test.js", 10, 7);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_cognitive_complexity_gatekeeper_violation() {
        let rust_code = r#"
fn deeply_nested_logic() {
    if true {
        if true {
            for _ in 0..10 {
                if true {
                    if true {
                        println!("deep");
                    }
                }
            }
        }
    }
}
"#;
        let lines: Vec<&str> = rust_code.lines().collect();
        let violations = GatekeeperScanner::check_function_spans(&lines, "src/nested.rs", 70, 3);
        assert!(violations.iter().any(|v| v.rule_identifier == "RULE_3_MAX_COGNITIVE_COMPLEXITY"));
    }

    #[test]
    fn test_banned_identifier_patterns_expanded() {
        assert!(BANNED_IDENTIFIER_PATTERN.is_match("let mut res = 1;"));
        assert!(BANNED_IDENTIFIER_PATTERN.is_match("data = fetch_results()"));
        assert!(BANNED_IDENTIFIER_PATTERN.is_match("res, err := doSomething()"));

        // Idiomatic patterns that MUST NOT be falsely banned
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("const { data } = response;"));
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("const { data, error } = supabase.from('x');"));
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("app.get('/', (req, res) => {});"));
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("response.data = fetch();"));
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("const userProfile = response;"));
        assert!(!BANNED_IDENTIFIER_PATTERN.is_match("let accountBalance = 100;"));
    }

    #[test]
    fn test_multilanguage_rule12_lazy_stubs_and_silent_catch() {
        let mut violations = Vec::new();

        // 1. Python silent catch & TODO
        let py_lines = vec!["try:", "    risky()", "except:", "    pass", "# TODO: fix later"];
        GatekeeperScanner::check_lazy_stubs_and_error_hygiene_rule(&py_lines, "service.py", "py", &mut violations);
        assert!(violations.iter().any(|v| v.rule_identifier == "RULE_12_NO_SILENT_CATCH"));
        assert!(violations.iter().any(|v| v.rule_identifier == "RULE_12_ANTI_LAZY_STUBS"));

        // 2. TypeScript silent catch
        violations.clear();
        let ts_lines = vec!["try { doWork(); } catch (err) {}", "const x = 42;"];
        GatekeeperScanner::check_lazy_stubs_and_error_hygiene_rule(&ts_lines, "handler.ts", "ts", &mut violations);
        assert!(violations.iter().any(|v| v.rule_identifier == "RULE_12_NO_SILENT_CATCH"));

        // 3. Rust unwrap in non-test file
        violations.clear();
        let rs_lines = vec!["let value = compute().unwrap();"];
        GatekeeperScanner::check_lazy_stubs_and_error_hygiene_rule(&rs_lines, "src/core.rs", "rs", &mut violations);
        assert!(violations.iter().any(|v| v.rule_identifier == "RULE_12_EXHAUSTIVE_ERROR_HANDLING"));
    }
}

