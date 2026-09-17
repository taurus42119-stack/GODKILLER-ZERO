use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static ANSI_ESCAPE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r"(?:\x1b\[[0-9;?]*[ -/]*[@-~]|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\r)") {
        Ok(compiled) => compiled,
        Err(_) => match Regex::new("") {
            Ok(fallback) => fallback,
            Err(_) => unreachable!(),
        },
    }
});

static PROGRESS_BAR_PATTERN: LazyLock<Regex> =
    LazyLock::new(
        || match Regex::new(r"(?m)^\s*\[[=>\-\s]{4,}\]\s*\d+%\s*.*$") {
            Ok(compiled) => compiled,
            Err(_) => match Regex::new("") {
                Ok(fallback) => fallback,
                Err(_) => unreachable!(),
            },
        },
    );

static TIMESTAMP_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"^(?:\[?\d{4}[-/.]\d{2}[-/.]\d{2}[T\s]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?(?:Z|[+-]\d{2}:?\d{2})?\]?\s*|\[\d{2}:\d{2}:\d{2}(?:[.,]\d+)?\]\s*)",
    ) {
        Ok(compiled) => compiled,
        Err(_) => match Regex::new("") {
            Ok(fallback) => fallback,
            Err(_) => unreachable!(),
        },
    }
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalOutputCategory {
    CargoTestPass,
    CargoTestFailure,
    NodeTestPass,
    NodeTestFailure,
    PytestPass,
    PytestFailure,
    GitDiff,
    BuildNoise,
    StackTraceOrLog,
    GenericPruned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruneResult {
    pub original_content: String,
    pub pruned_content: String,
    pub original_tokens_est: usize,
    pub pruned_tokens_est: usize,
    pub reduction_percentage: f64,
    pub category: TerminalOutputCategory,
}

pub struct TerminalPruner;

impl TerminalPruner {
    #[must_use]
    pub fn strip_ansi_escapes(raw_output: &str) -> String {
        ANSI_ESCAPE_PATTERN.replace_all(raw_output, "").to_string()
    }

    fn parse_cargo_test_ok_line(line: &str, total_passed: &mut usize, elapsed_time: &mut String) {
        if !line.contains("test result: ok.") {
            return;
        }
        let parsed_num = line
            .split("ok.")
            .nth(1)
            .and_then(|s| s.split("passed;").next())
            .and_then(|s| s.trim().parse::<usize>().ok());

        if let Some(num) = parsed_num {
            *total_passed += num;
        }
        if let Some(time_part) = line.split("finished in ").nth(1) {
            *elapsed_time = time_part.trim().to_string();
        }
    }

    fn prune_cargo_test_success(clean_text: &str) -> (String, TerminalOutputCategory) {
        let mut total_passed = 0usize;
        let mut elapsed_time = String::new();

        for line in clean_text.lines() {
            Self::parse_cargo_test_ok_line(line, &mut total_passed, &mut elapsed_time);
        }

        let summary = if !elapsed_time.is_empty() {
            format!(
                "[OK] [CARGO_TEST_VERIFIED] {} passed in {} (All test assertions green)",
                total_passed, elapsed_time
            )
        } else {
            format!(
                "[OK] [CARGO_TEST_VERIFIED] {} passed (All test assertions green)",
                total_passed
            )
        };

        (summary, TerminalOutputCategory::CargoTestPass)
    }

    fn check_failure_line(lk_trim: &str, reason: &mut String, location: &mut String) -> bool {
        if lk_trim.starts_with("---- ") || lk_trim.starts_with("test result:") {
            return true;
        }
        if lk_trim.contains("panicked at") {
            *reason = lk_trim.to_string();
        } else if lk_trim.starts_with("location: ") || lk_trim.contains("location:") {
            *location = lk_trim.to_string();
        } else if lk_trim.starts_with("assertion `left == right` failed")
            || lk_trim.starts_with("assertion failed:")
        {
            if reason.is_empty() {
                *reason = lk_trim.to_string();
            } else {
                reason.push_str(&format!(" | {}", lk_trim));
            }
        }
        false
    }

    fn parse_single_failure_details(lines: &[&str], start_idx: usize) -> (String, String) {
        let mut failure_reason = String::new();
        let mut failure_location = String::new();

        for &lookahead in lines.iter().skip(start_idx).take(25) {
            if Self::check_failure_line(
                lookahead.trim(),
                &mut failure_reason,
                &mut failure_location,
            ) {
                break;
            }
        }
        (failure_reason, failure_location)
    }

    fn parse_cargo_failure_line(
        line: &str,
        idx: usize,
        lines: &[&str],
        in_failures_section: &mut bool,
        failures: &mut Vec<(String, String, String)>,
    ) {
        let trimmed = line.trim();
        if trimmed == "failures:" {
            *in_failures_section = true;
            return;
        }
        if !*in_failures_section {
            return;
        }
        if trimmed.starts_with("test result:") {
            *in_failures_section = false;
            return;
        }
        if trimmed.starts_with("---- ") && trimmed.ends_with(" stdout ----") {
            let test_name = trimmed
                .trim_start_matches("---- ")
                .trim_end_matches(" stdout ----");

            let (reason, location) = Self::parse_single_failure_details(lines, idx + 1);
            failures.push((test_name.to_string(), reason, location));
        }
    }

    fn extract_cargo_failures(lines: &[&str]) -> Vec<(String, String, String)> {
        let mut failures = Vec::new();
        let mut in_failures_section = false;

        for (idx, &line) in lines.iter().enumerate() {
            Self::parse_cargo_failure_line(
                line,
                idx,
                lines,
                &mut in_failures_section,
                &mut failures,
            );
        }
        failures
    }

    fn format_cargo_failures_fallback(lines: &[&str], output: &mut String) {
        for &line in lines {
            let tr = line.trim();
            if tr.contains("FAILED") || tr.contains("panicked at") || tr.starts_with("error:") {
                output.push_str(&format!("  • {}\n", tr));
            }
        }
    }

    fn format_cargo_failures_list(failures: Vec<(String, String, String)>, output: &mut String) {
        for (name, reason, loc) in failures {
            output.push_str(&format!("  • Test: {}\n", name));
            if !loc.is_empty() {
                output.push_str(&format!("    Location: {}\n", loc));
            }
            if !reason.is_empty() {
                output.push_str(&format!("    Reason: {}\n", reason));
            }
        }
    }

    fn format_cargo_failures(
        failures: Vec<(String, String, String)>,
        lines: &[&str],
    ) -> (String, TerminalOutputCategory) {
        let mut output = String::new();
        output.push_str(&format!(
            "[FAIL] [CARGO_TEST_FAILURE] {} failure(s) isolated\n",
            failures.len().max(1)
        ));
        output.push_str("FAILURES:\n");

        if failures.is_empty() {
            Self::format_cargo_failures_fallback(lines, &mut output);
        } else {
            Self::format_cargo_failures_list(failures, &mut output);
        }

        (
            output.trim_end().to_string(),
            TerminalOutputCategory::CargoTestFailure,
        )
    }

    #[must_use]
    pub fn prune_cargo_test(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let has_cargo_markers =
            clean_text.contains("running ") && clean_text.contains("test result:");
        if !has_cargo_markers {
            return None;
        }

        let is_success = clean_text.contains("test result: ok.") && !clean_text.contains("FAILED");
        if is_success {
            return Some(Self::prune_cargo_test_success(clean_text));
        }

        let lines: Vec<&str> = clean_text.lines().collect();
        let failures = Self::extract_cargo_failures(&lines);
        Some(Self::format_cargo_failures(failures, &lines))
    }

    fn extract_node_test_success(clean_text: &str) -> (String, TerminalOutputCategory) {
        for line in clean_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Tests:") && trimmed.contains("passed") {
                return (
                    format!(
                        "[OK] [NODE_TEST_VERIFIED] {} (All test assertions green)",
                        trimmed
                    ),
                    TerminalOutputCategory::NodeTestPass,
                );
            }
        }
        (
            "[OK] [NODE_TEST_VERIFIED] All tests passed cleanly".into(),
            TerminalOutputCategory::NodeTestPass,
        )
    }

    fn capture_node_failure_line<'a>(
        line: &'a str,
        capture: &mut bool,
        isolated: &mut Vec<&'a str>,
    ) -> bool {
        let tr = line.trim();
        if tr.starts_with("FAIL ") || tr.starts_with("● ") {
            *capture = true;
        }
        if *capture {
            isolated.push(line);
            if tr.starts_with("Tests:") {
                return true;
            }
        }
        false
    }

    fn extract_node_test_failure(clean_text: &str) -> (String, TerminalOutputCategory) {
        let mut isolated = Vec::new();
        let mut capture = false;
        for line in clean_text.lines() {
            if Self::capture_node_failure_line(line, &mut capture, &mut isolated) {
                break;
            }
        }

        let isolated_output = if isolated.is_empty() {
            clean_text.to_string()
        } else {
            format!("[FAIL] [NODE_TEST_FAILURE]\n{}", isolated.join("\n"))
        };

        (isolated_output, TerminalOutputCategory::NodeTestFailure)
    }

    #[must_use]
    pub fn prune_node_test(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_node_test = clean_text.contains("Tests:")
            || clean_text.contains("Test Suites:")
            || clean_text.contains("PASS ")
            || clean_text.contains("FAIL ");
        if !is_node_test {
            return None;
        }

        let has_failures = clean_text.contains("FAIL ") || clean_text.contains("failed,");
        if !has_failures {
            return Some(Self::extract_node_test_success(clean_text));
        }

        Some(Self::extract_node_test_failure(clean_text))
    }

    fn extract_pytest_success(clean_text: &str) -> (String, TerminalOutputCategory) {
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("===") && tr.contains("passed") {
                let cleaned = tr.trim_matches('=').trim();
                return (
                    format!(
                        "[OK] [PYTEST_VERIFIED] {} (All test assertions green)",
                        cleaned
                    ),
                    TerminalOutputCategory::PytestPass,
                );
            }
        }
        (
            "[OK] [PYTEST_VERIFIED] All python tests passed cleanly".into(),
            TerminalOutputCategory::PytestPass,
        )
    }

    fn extract_pytest_failure(clean_text: &str) -> (String, TerminalOutputCategory) {
        let mut failure_lines = Vec::new();
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("FAILED ")
                || tr.starts_with("E   ")
                || tr.starts_with("AssertionError")
                || tr.starts_with("===")
            {
                failure_lines.push(line);
            }
        }
        (
            format!("[FAIL] [PYTEST_FAILURE]\n{}", failure_lines.join("\n")),
            TerminalOutputCategory::PytestFailure,
        )
    }

    #[must_use]
    pub fn prune_pytest(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_pytest = clean_text.contains("pytest")
            || (clean_text.contains("passed in") && clean_text.contains("===="));
        if !is_pytest {
            return None;
        }

        let has_failures = clean_text.contains("FAILED ")
            || clean_text.contains("failed in")
            || clean_text.contains("ERRORS ===");
        if !has_failures {
            return Some(Self::extract_pytest_success(clean_text));
        }

        Some(Self::extract_pytest_failure(clean_text))
    }

    #[must_use]
    pub fn prune_git_diff(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        if !clean_text.contains("diff --git ") && !clean_text.contains("@@ -") {
            return None;
        }

        let mut pruned_lines = Vec::new();
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("index ") && tr.contains("..") {
                continue;
            }
            if tr.starts_with("--- a/") || tr.starts_with("+++ b/") {
                continue;
            }
            pruned_lines.push(line);
        }

        Some((pruned_lines.join("\n"), TerminalOutputCategory::GitDiff))
    }

    fn filter_build_lines<'a>(
        clean_text: &'a str,
        had_compiling: &mut bool,
        significant_lines: &mut Vec<&'a str>,
    ) {
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("Compiling ") {
                *had_compiling = true;
                continue;
            }
            if tr.starts_with("Downloading ") || tr.starts_with("Fetching ") {
                continue;
            }
            significant_lines.push(line);
        }
    }

    #[must_use]
    pub fn prune_build_output(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_build = clean_text.contains("Compiling ") || clean_text.contains("Finished `");
        if !is_build {
            return None;
        }

        let mut significant_lines = Vec::new();
        let mut had_compiling = false;

        Self::filter_build_lines(clean_text, &mut had_compiling, &mut significant_lines);

        if had_compiling && significant_lines.is_empty() {
            return Some((
                "[OK] [BUILD_CLEAN] Compilation completed with zero warnings or errors.".into(),
                TerminalOutputCategory::BuildNoise,
            ));
        }

        if had_compiling {
            return Some((
                significant_lines.join("\n"),
                TerminalOutputCategory::BuildNoise,
            ));
        }

        None
    }

    fn is_error_keyword(line: &str) -> bool {
        let upper = line.to_ascii_uppercase();
        upper.contains("ERROR:")
            || upper.contains("[ERROR]")
            || upper.contains("FATAL:")
            || upper.contains("[FATAL]")
            || upper.contains("CRITICAL:")
            || upper.contains("[CRITICAL]")
            || upper.contains("PANIC")
            || upper.contains("EXCEPTION:")
            || upper.contains("TRACEBACK (MOST RECENT CALL LAST):")
            || (upper.contains("ERROR")
                && (upper.contains("FAILED") || upper.contains("EXCEPTION")))
    }

    fn is_debug_or_trace_noise(line: &str) -> bool {
        let trimmed = line.trim();
        let upper = trimmed.to_ascii_uppercase();
        upper.starts_with("DEBUG:")
            || upper.starts_with("[DEBUG]")
            || upper.starts_with("TRACE:")
            || upper.starts_with("[TRACE]")
            || upper.starts_with("DEBUG ")
            || upper.starts_with("TRACE ")
    }

    fn is_framework_stack_frame(line: &str) -> Option<&'static str> {
        let trimmed = line.trim();
        if trimmed.starts_with("at Microsoft.") || trimmed.starts_with("at System.") {
            return Some("Microsoft / .NET Core");
        }
        if trimmed.starts_with("at org.springframework.")
            || trimmed.starts_with("at java.")
            || trimmed.starts_with("at javax.")
            || trimmed.starts_with("at sun.")
        {
            return Some("Java / Spring Framework");
        }
        if trimmed.contains("node:internal")
            || trimmed.contains("(node:")
            || trimmed.contains("node_modules\\")
            || trimmed.contains("node_modules/")
        {
            return Some("Node.js / npm internals");
        }
        if trimmed.contains("site-packages")
            || trimmed.contains("dist-packages")
            || trimmed.contains("lib/python")
            || trimmed.contains("lib\\python")
            || trimmed.contains("<frozen ")
        {
            return Some("Python internals");
        }
        None
    }

    #[must_use]
    fn has_log_or_stack_signature(clean_text: &str) -> bool {
        let has_timestamp = clean_text
            .lines()
            .any(|l| TIMESTAMP_PATTERN.is_match(l.trim_start()));
        let has_stack_trace = clean_text.lines().any(|l| {
            let tr = l.trim_start();
            tr.starts_with("at ") || tr.starts_with("File \"") || tr.starts_with("Traceback")
        });
        let has_log_levels = clean_text.lines().any(|l| {
            let tr = l.trim();
            tr.contains("ERROR:")
                || tr.contains("[ERROR]")
                || tr.contains("WARN:")
                || tr.contains("[WARN]")
                || tr.contains("INFO:")
                || tr.contains("[INFO]")
                || tr.contains("DEBUG:")
                || tr.contains("[DEBUG]")
                || tr.contains("FATAL:")
                || tr.contains("[FATAL]")
                || tr.contains("Exception:")
        });
        has_timestamp || has_stack_trace || has_log_levels
    }

    fn strip_timestamps_and_filter_noise(raw_lines: &[&str], has_error: bool) -> Vec<String> {
        let mut stripped_lines: Vec<String> = Vec::new();
        for &line in raw_lines {
            let leading_spaces: String = line
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t')
                .collect();
            let tr = line.trim_start();
            let without_ts = if TIMESTAMP_PATTERN.is_match(tr) {
                format!("{}{}", leading_spaces, TIMESTAMP_PATTERN.replace(tr, ""))
            } else {
                line.to_string()
            };

            if has_error && Self::is_debug_or_trace_noise(&without_ts) {
                continue;
            }

            stripped_lines.push(without_ts);
        }
        stripped_lines
    }

    fn flush_framework_frames(
        collapsed_frames: &mut Vec<String>,
        framework_count: &mut usize,
        current_framework_category: &str,
        indent: &str,
    ) {
        if *framework_count > 0 {
            collapsed_frames.push(format!(
                "{}... [{} framework frames collapsed: {}] ...",
                indent, *framework_count, current_framework_category
            ));
            *framework_count = 0;
        }
    }

    fn collapse_framework_stack_frames(stripped_lines: Vec<String>) -> Vec<String> {
        let mut collapsed_frames: Vec<String> = Vec::new();
        let mut framework_count = 0usize;
        let mut current_framework_category = "";
        let mut in_python_framework_frame = false;

        for line in stripped_lines {
            if let Some(fw_name) = Self::is_framework_stack_frame(&line) {
                framework_count += 1;
                current_framework_category = fw_name;
                in_python_framework_frame = fw_name == "Python internals";
            } else if in_python_framework_frame
                && !line.trim_start().starts_with("File \"")
                && !Self::is_error_keyword(&line)
                && !line.trim().is_empty()
            {
                in_python_framework_frame = false;
            } else {
                in_python_framework_frame = false;
                let indent = if line.starts_with("   ") {
                    "   "
                } else {
                    "    "
                };
                Self::flush_framework_frames(
                    &mut collapsed_frames,
                    &mut framework_count,
                    current_framework_category,
                    indent,
                );
                collapsed_frames.push(line);
            }
        }
        Self::flush_framework_frames(
            &mut collapsed_frames,
            &mut framework_count,
            current_framework_category,
            "   ",
        );
        collapsed_frames
    }

    fn push_repeated_line(final_lines: &mut Vec<String>, prev: String, repeat_count: usize) {
        if repeat_count > 1 {
            final_lines.push(format!("{} (repeated {} times)", prev, repeat_count));
        } else {
            final_lines.push(prev);
        }
    }

    fn step_dedup_line(
        line: String,
        prev_line: &mut Option<String>,
        repeat_count: &mut usize,
        final_lines: &mut Vec<String>,
    ) {
        let tr = line.trim();
        let is_both_empty =
            tr.is_empty() && prev_line.as_ref().is_some_and(|p| p.trim().is_empty());
        if is_both_empty {
            return;
        }

        let is_same_repeated = prev_line.as_ref().is_some_and(|p| p == &line) && !tr.is_empty();
        if is_same_repeated {
            *repeat_count += 1;
            return;
        }

        if let Some(prev_taken) = prev_line.take() {
            Self::push_repeated_line(final_lines, prev_taken, *repeat_count);
            *repeat_count = 1;
        }
        *prev_line = Some(line);
    }

    fn deduplicate_consecutive_lines(collapsed_frames: Vec<String>) -> Vec<String> {
        let mut final_lines: Vec<String> = Vec::new();
        let mut repeat_count = 1usize;
        let mut prev_line: Option<String> = None;

        for line in collapsed_frames {
            Self::step_dedup_line(line, &mut prev_line, &mut repeat_count, &mut final_lines);
        }

        if let Some(prev) = prev_line {
            Self::push_repeated_line(&mut final_lines, prev, repeat_count);
        }
        final_lines
    }

    #[must_use]
    pub fn prune_log_and_stack_trace(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        if !Self::has_log_or_stack_signature(clean_text) {
            return None;
        }

        let raw_lines: Vec<&str> = clean_text.lines().collect();
        if raw_lines.is_empty() {
            return None;
        }

        let has_error = raw_lines.iter().any(|l| Self::is_error_keyword(l));
        let stripped_lines = Self::strip_timestamps_and_filter_noise(&raw_lines, has_error);
        let collapsed_frames = Self::collapse_framework_stack_frames(stripped_lines);
        let final_lines = Self::deduplicate_consecutive_lines(collapsed_frames);

        let result = final_lines.join("\n");
        if result.len() < clean_text.len() {
            Some((result, TerminalOutputCategory::StackTraceOrLog))
        } else {
            None
        }
    }

    fn prune_generic_fallback(progress_cleaned: &str) -> (String, TerminalOutputCategory) {
        let mut compact_lines = Vec::new();
        let mut last_line = "";
        for line in progress_cleaned.lines() {
            let tr = line.trim();
            if tr.is_empty() && last_line.is_empty() {
                continue;
            }
            compact_lines.push(line);
            last_line = tr;
        }
        (
            compact_lines.join("\n"),
            TerminalOutputCategory::GenericPruned,
        )
    }

    #[must_use]
    pub fn prune(raw_output: &str) -> PruneResult {
        let stripped_ansi = Self::strip_ansi_escapes(raw_output);
        let progress_cleaned = PROGRESS_BAR_PATTERN
            .replace_all(&stripped_ansi, "")
            .to_string();

        let (pruned, category) = if let Some(pruned_candidate) =
            Self::prune_cargo_test(&progress_cleaned)
        {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_node_test(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_pytest(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_git_diff(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_build_output(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_log_and_stack_trace(&progress_cleaned) {
            pruned_candidate
        } else {
            Self::prune_generic_fallback(&progress_cleaned)
        };

        let orig_len = stripped_ansi.len().max(1);
        let pruned_len = pruned.len();

        let reduction_percentage = if pruned_len >= orig_len {
            0.0
        } else {
            ((orig_len - pruned_len) as f64 / orig_len as f64) * 100.0
        };

        let original_tokens_est = (orig_len / 4).max(1);
        let pruned_tokens_est = (pruned_len / 4).max(1);

        PruneResult {
            original_content: stripped_ansi,
            pruned_content: pruned,
            original_tokens_est,
            pruned_tokens_est,
            reduction_percentage,
            category,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_escapes() {
        let raw = "\x1b[32mtest antigravity::hook::test ... \x1b[0m\x1b[1mok\x1b[0m\r\n";
        let cleaned = TerminalPruner::strip_ansi_escapes(raw);
        assert_eq!(cleaned, "test antigravity::hook::test ... ok\n");
    }

    #[test]
    fn test_prune_cargo_test_success_folds_to_single_line() {
        let raw = "
running 52 tests
test test_one ... ok
test test_two ... ok
test test_three ... ok

test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.42s
";
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(
            pruned_outcome.category,
            TerminalOutputCategory::CargoTestPass
        );
        assert!(pruned_outcome.pruned_content.contains("52 passed in 1.42s"));
        assert!(pruned_outcome.reduction_percentage > 50.0);
    }

    #[test]
    fn test_prune_cargo_test_failure_isolates_error_reason() {
        let raw = "
running 3 tests
test test_ok_1 ... ok
test test_ok_2 ... ok
test test_broken ... FAILED

failures:

---- test_broken stdout ----
thread 'test_broken' panicked at tests/sample_test.rs:42:5:
assertion `left == right` failed
  left: 75
 right: 70
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    test_broken

test result: FAILED. 2 passed; 1 failed; 0 ignored; finished in 0.05s
";
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(
            pruned_outcome.category,
            TerminalOutputCategory::CargoTestFailure
        );
        assert!(pruned_outcome.pruned_content.contains("test_broken"));
        assert!(pruned_outcome
            .pruned_content
            .contains("assertion `left == right` failed"));
        // Ensure successful tests are pruned out of failure report
        assert!(!pruned_outcome.pruned_content.contains("test_ok_1 ... ok"));
    }

    #[test]
    fn test_prune_node_test_success() {
        let raw = "
 PASS  src/components/Header.test.tsx
 PASS  src/utils/math.test.ts

Test Suites: 2 passed, 2 total
Tests:       14 passed, 14 total
Snapshots:   0 total
Time:        1.234 s
";
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(
            pruned_outcome.category,
            TerminalOutputCategory::NodeTestPass
        );
        assert!(pruned_outcome
            .pruned_content
            .contains("Tests:       14 passed, 14 total"));
    }

    #[test]
    fn test_prune_pytest_success() {
        let raw = "
============================= test session starts =============================
platform win32 -- Python 3.11.0, pytest-7.4.0
rootdir: C:\\projects\\sample
collected 10 items

test_app.py ..........                                                   [100%]

============================== 10 passed in 0.35s ==============================
";
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(pruned_outcome.category, TerminalOutputCategory::PytestPass);
        assert!(pruned_outcome.pruned_content.contains("10 passed in 0.35s"));
    }

    #[test]
    fn test_prune_git_diff_strips_redundant_index_hashes() {
        let raw = "
diff --git a/src/main.rs b/src/main.rs
index abc1234..def5678 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -10,4 +10,4 @@
-let old = 1;
+let new = 2;
";
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(pruned_outcome.category, TerminalOutputCategory::GitDiff);
        assert!(!pruned_outcome
            .pruned_content
            .contains("index abc1234..def5678"));
        assert!(pruned_outcome.pruned_content.contains("+let new = 2;"));
    }

    #[test]
    fn test_prune_log_and_stack_trace_filters_noise_and_collapses_framework_frames() {
        let raw = r#"
[2024-01-01 12:00:00.123] DEBUG: Processing request id=abc123
[2024-01-01 12:00:00.124] DEBUG: Cache miss for key=user_session
[2024-01-01 12:00:00.125] INFO: Connecting to database pool
[2024-01-01 12:00:00.126] DEBUG: SQL: SELECT * FROM users WHERE id=1
[2024-01-01 12:00:00.127] DEBUG: Acquired connection in 1.2ms
[2024-01-01 12:00:00.128] ERROR: NullReferenceException: Object reference not set to an instance of an object
   at MyApp.Services.UserService.GetUser(Int32 id) in C:\repo\src\UserService.cs:line 45
   at MyApp.Controllers.UserController.Get(Int32 id) in C:\repo\src\UserController.cs:line 23
   at Microsoft.AspNetCore.Mvc.Infrastructure.ActionMethodExecutor.SyncActionResultExecutor.Execute(IActionResultTypeMapper mapper, ObjectMethodExecutor executor, Object controller, Object[] arguments)
   at Microsoft.AspNetCore.Mvc.Infrastructure.ControllerActionInvoker.InvokeActionMethodAsync()
   at Microsoft.AspNetCore.Mvc.Infrastructure.ControllerActionInvoker.Next(State& next, Scope& scope, Object& state, Boolean& isCompleted)
   at Microsoft.AspNetCore.Mvc.Infrastructure.ResourceInvoker.g__Awaited|20_0(ResourceInvoker invoker, Task lastTask, State next, Scope scope, Object state, Boolean isCompleted)
   at Microsoft.AspNetCore.Routing.EndpointMiddleware.g__AwaitViaAwaitable|8_0(EndpointMiddleware middleware, HttpContext httpContext, Task task)
   at Microsoft.AspNetCore.Server.Kestrel.Core.Internal.Http.HttpProtocol.ProcessRequests[TContext](IHttpApplication`1 application)
[2024-01-01 12:00:00.130] DEBUG: Request completed in 7ms with status 500
"#;
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(
            pruned_outcome.category,
            TerminalOutputCategory::StackTraceOrLog
        );
        assert!(pruned_outcome
            .pruned_content
            .contains("INFO: Connecting to database pool"));
        assert!(pruned_outcome
            .pruned_content
            .contains("ERROR: NullReferenceException"));
        assert!(pruned_outcome
            .pruned_content
            .contains("MyApp.Services.UserService.GetUser"));
        assert!(pruned_outcome
            .pruned_content
            .contains("framework frames collapsed: Microsoft / .NET Core"));
        // DEBUG noise stripped
        assert!(!pruned_outcome.pruned_content.contains("DEBUG: SQL: SELECT"));
        assert!(!pruned_outcome
            .pruned_content
            .contains("DEBUG: Processing request"));
        // Significant token reduction (> 50%)
        assert!(pruned_outcome.reduction_percentage > 50.0);
    }

    #[test]
    fn test_prune_repeated_lines_folding() {
        let raw = r#"
[2024-01-01 10:00:00] INFO: Waiting for worker connection...
[2024-01-01 10:00:01] INFO: Waiting for worker connection...
[2024-01-01 10:00:02] INFO: Waiting for worker connection...
[2024-01-01 10:00:03] INFO: Waiting for worker connection...
[2024-01-01 10:00:04] INFO: Connected successfully!
"#;
        let pruned_outcome = TerminalPruner::prune(raw);
        assert_eq!(
            pruned_outcome.category,
            TerminalOutputCategory::StackTraceOrLog
        );
        assert!(pruned_outcome
            .pruned_content
            .contains("INFO: Waiting for worker connection... (repeated 4 times)"));
        assert!(pruned_outcome
            .pruned_content
            .contains("INFO: Connected successfully!"));
    }
}
