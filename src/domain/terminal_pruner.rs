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

static PROGRESS_BAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(r"(?m)^\s*\[[=>\-\s]{4,}\]\s*\d+%\s*.*$") {
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

    fn prune_cargo_test_success(clean_text: &str) -> (String, TerminalOutputCategory) {
        let mut total_passed = 0usize;
        let mut elapsed_time = String::new();

        for line in clean_text.lines() {
            if line.contains("test result: ok.") {
                if let Some(passed_str) = line.split("ok.").nth(1) {
                    if let Some(num_str) = passed_str.split("passed;").next() {
                        if let Ok(num) = num_str.trim().parse::<usize>() {
                            total_passed += num;
                        }
                    }
                }
                if let Some(time_part) = line.split("finished in ").nth(1) {
                    elapsed_time = time_part.trim().to_string();
                }
            }
        }

        let summary = if !elapsed_time.is_empty() {
            format!(
                "✓ [CARGO_TEST_VERIFIED] {} passed in {} (All test assertions green)",
                total_passed, elapsed_time
            )
        } else {
            format!(
                "✓ [CARGO_TEST_VERIFIED] {} passed (All test assertions green)",
                total_passed
            )
        };

        (summary, TerminalOutputCategory::CargoTestPass)
    }

    fn extract_cargo_failures(lines: &[&str]) -> Vec<(String, String, String)> {
        let mut failures = Vec::new();
        let mut in_failures_section = false;

        for (idx, &line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed == "failures:" {
                in_failures_section = true;
                continue;
            }

            if in_failures_section {
                if trimmed.starts_with("test result:") {
                    in_failures_section = false;
                    continue;
                }
                if trimmed.starts_with("---- ") && trimmed.ends_with(" stdout ----") {
                    let test_name = trimmed
                        .trim_start_matches("---- ")
                        .trim_end_matches(" stdout ----");

                    let mut failure_reason = String::new();
                    let mut failure_location = String::new();

                    for &lookahead in lines.iter().skip(idx + 1).take(25) {
                        let lk_trim = lookahead.trim();
                        if lk_trim.starts_with("---- ") || lk_trim.starts_with("test result:") {
                            break;
                        }
                        if lk_trim.contains("panicked at") {
                            failure_reason = lk_trim.to_string();
                        } else if lk_trim.starts_with("location: ") || lk_trim.contains("location:") {
                            failure_location = lk_trim.to_string();
                        } else if lk_trim.starts_with("assertion `left == right` failed")
                            || lk_trim.starts_with("assertion failed:")
                        {
                            if failure_reason.is_empty() {
                                failure_reason = lk_trim.to_string();
                            } else {
                                failure_reason.push_str(&format!(" | {}", lk_trim));
                            }
                        }
                    }

                    failures.push((test_name.to_string(), failure_reason, failure_location));
                }
            }
        }
        failures
    }

    fn format_cargo_failures(failures: Vec<(String, String, String)>, lines: &[&str]) -> (String, TerminalOutputCategory) {
        let mut output = String::new();
        output.push_str(&format!("❌ [CARGO_TEST_FAILURE] {} failure(s) isolated\n", failures.len().max(1)));
        output.push_str("FAILURES:\n");

        if failures.is_empty() {
            for &line in lines {
                let tr = line.trim();
                if tr.contains("FAILED") || tr.contains("panicked at") || tr.starts_with("error:") {
                    output.push_str(&format!("  • {}\n", tr));
                }
            }
        } else {
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

        (output.trim_end().to_string(), TerminalOutputCategory::CargoTestFailure)
    }

    #[must_use]
    pub fn prune_cargo_test(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let has_cargo_markers = clean_text.contains("running ") && clean_text.contains("test result:");
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

    #[must_use]
    pub fn prune_node_test(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_node_test = clean_text.contains("Tests:") || clean_text.contains("Test Suites:") || clean_text.contains("PASS ") || clean_text.contains("FAIL ");
        if !is_node_test {
            return None;
        }

        let has_failures = clean_text.contains("FAIL ") || clean_text.contains("failed,");
        if !has_failures {
            for line in clean_text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Tests:") && trimmed.contains("passed") {
                    return Some((
                        format!("✓ [NODE_TEST_VERIFIED] {} (All test assertions green)", trimmed),
                        TerminalOutputCategory::NodeTestPass,
                    ));
                }
            }
            return Some((
                "✓ [NODE_TEST_VERIFIED] All tests passed cleanly".into(),
                TerminalOutputCategory::NodeTestPass,
            ));
        }

        let mut isolated = Vec::new();
        let mut capture = false;
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("FAIL ") || tr.starts_with("● ") {
                capture = true;
            }
            if capture {
                isolated.push(line);
                if tr.starts_with("Tests:") {
                    break;
                }
            }
        }

        let isolated_output = if isolated.is_empty() {
            clean_text.to_string()
        } else {
            format!("❌ [NODE_TEST_FAILURE]\n{}", isolated.join("\n"))
        };

        Some((isolated_output, TerminalOutputCategory::NodeTestFailure))
    }

    #[must_use]
    pub fn prune_pytest(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_pytest = clean_text.contains("pytest") || (clean_text.contains("passed in") && clean_text.contains("===="));
        if !is_pytest {
            return None;
        }

        let has_failures = clean_text.contains("FAILED ") || clean_text.contains("failed in") || clean_text.contains("ERRORS ===");
        if !has_failures {
            for line in clean_text.lines() {
                let tr = line.trim();
                if tr.starts_with("===") && tr.contains("passed") {
                    let cleaned = tr.trim_matches('=').trim();
                    return Some((
                        format!("✓ [PYTEST_VERIFIED] {} (All test assertions green)", cleaned),
                        TerminalOutputCategory::PytestPass,
                    ));
                }
            }
            return Some((
                "✓ [PYTEST_VERIFIED] All python tests passed cleanly".into(),
                TerminalOutputCategory::PytestPass,
            ));
        }

        let mut failure_lines = Vec::new();
        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("FAILED ") || tr.starts_with("E   ") || tr.starts_with("AssertionError") || tr.starts_with("===") {
                failure_lines.push(line);
            }
        }

        let failure_summary = format!("❌ [PYTEST_FAILURE]\n{}", failure_lines.join("\n"));
        Some((failure_summary, TerminalOutputCategory::PytestFailure))
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

    #[must_use]
    pub fn prune_build_output(clean_text: &str) -> Option<(String, TerminalOutputCategory)> {
        let is_build = clean_text.contains("Compiling ") || clean_text.contains("Finished `");
        if !is_build {
            return None;
        }

        let mut significant_lines = Vec::new();
        let mut had_compiling = false;

        for line in clean_text.lines() {
            let tr = line.trim();
            if tr.starts_with("Compiling ") {
                had_compiling = true;
                continue;
            }
            if tr.starts_with("Downloading ") || tr.starts_with("Fetching ") {
                continue;
            }
            significant_lines.push(line);
        }

        if had_compiling && significant_lines.is_empty() {
            return Some((
                "✓ [BUILD_CLEAN] Compilation completed with zero warnings or errors.".into(),
                TerminalOutputCategory::BuildNoise,
            ));
        }

        if had_compiling {
            return Some((significant_lines.join("\n"), TerminalOutputCategory::BuildNoise));
        }

        None
    }

    #[must_use]
    pub fn prune(raw_output: &str) -> PruneResult {
        let stripped_ansi = Self::strip_ansi_escapes(raw_output);
        let progress_cleaned = PROGRESS_BAR_PATTERN.replace_all(&stripped_ansi, "").to_string();

        let (pruned, category) = if let Some(pruned_candidate) = Self::prune_cargo_test(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_node_test(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_pytest(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_git_diff(&progress_cleaned) {
            pruned_candidate
        } else if let Some(pruned_candidate) = Self::prune_build_output(&progress_cleaned) {
            pruned_candidate
        } else {
            // Generic compression: collapse multiple blank lines and dedup identical consecutive lines
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
            (compact_lines.join("\n"), TerminalOutputCategory::GenericPruned)
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
        assert_eq!(pruned_outcome.category, TerminalOutputCategory::CargoTestPass);
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
        assert_eq!(pruned_outcome.category, TerminalOutputCategory::CargoTestFailure);
        assert!(pruned_outcome.pruned_content.contains("test_broken"));
        assert!(pruned_outcome.pruned_content.contains("assertion `left == right` failed"));
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
        assert_eq!(pruned_outcome.category, TerminalOutputCategory::NodeTestPass);
        assert!(pruned_outcome.pruned_content.contains("Tests:       14 passed, 14 total"));
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
        assert!(!pruned_outcome.pruned_content.contains("index abc1234..def5678"));
        assert!(pruned_outcome.pruned_content.contains("+let new = 2;"));
    }
}
