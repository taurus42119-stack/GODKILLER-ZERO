use regex::Regex;
use std::sync::LazyLock;

static INJECTION_TOKEN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    match Regex::new(
        r"(?i)(<\|endoftext\|>|\[INST\]|\[/INST\]|<fim_prefix>|<fim_suffix>|system:\s*override)",
    ) {
        Ok(compiled) => compiled,
        Err(_) => match Regex::new("") {
            Ok(fallback) => fallback,
            Err(_) => unreachable!(),
        },
    }
});

static ANTIGRAVITY_USER_REQUEST_PATTERN: LazyLock<Regex> =
    LazyLock::new(
        || match Regex::new(r"(?s)<USER_REQUEST>(.*?)</USER_REQUEST>") {
            Ok(compiled) => compiled,
            Err(_) => match Regex::new("") {
                Ok(fallback) => fallback,
                Err(_) => unreachable!(),
            },
        },
    );

pub struct IngressSecuritySanitizer;

impl IngressSecuritySanitizer {
    #[must_use]
    pub fn sanitize_prompt_tokens(input_text: &str) -> String {
        INJECTION_TOKEN_PATTERN
            .replace_all(input_text, "[SANITIZED_TOKEN]")
            .to_string()
    }

    #[must_use]
    pub fn extract_isolated_user_request(input_payload: &str) -> (Option<String>, bool) {
        if let Some(captures) = ANTIGRAVITY_USER_REQUEST_PATTERN.captures(input_payload) {
            if let Some(matched_request) = captures.get(1) {
                return (Some(matched_request.as_str().trim().to_string()), true);
            }
        }
        (None, false)
    }

    #[must_use]
    pub fn inject_compiled_request_into_antigravity_envelope(
        original_envelope: &str,
        compiled_ir: &str,
    ) -> String {
        ANTIGRAVITY_USER_REQUEST_PATTERN
            .replace(
                original_envelope,
                format!("<USER_REQUEST>\n{}\n</USER_REQUEST>", compiled_ir),
            )
            .to_string()
    }

    fn parse_origin_authority(authority: &str) -> Result<(&str, &str), ()> {
        if authority.starts_with('[') {
            let end_bracket = authority.find(']').ok_or(())?;
            let host_part = &authority[..=end_bracket];
            let rest = &authority[end_bracket + 1..];
            let port_part = rest.strip_prefix(':').unwrap_or("");
            Ok((host_part, port_part))
        } else {
            let mut parts = authority.split(':');
            let host_part = parts.next().unwrap_or("");
            let port_part = parts.next().unwrap_or("");
            Ok((host_part, port_part))
        }
    }

    #[must_use]
    pub fn is_browser_origin_forbidden(origin_header: Option<&str>) -> bool {
        let Some(origin) = origin_header else {
            return false;
        };
        let lower_origin = origin.trim().to_lowercase();
        if lower_origin.is_empty() {
            return false;
        }

        let without_scheme = if let Some(stripped) = lower_origin.strip_prefix("http://") {
            stripped
        } else if let Some(stripped) = lower_origin.strip_prefix("https://") {
            stripped
        } else {
            return true;
        };

        let authority = without_scheme.split('/').next().unwrap_or("");
        let Ok((host, port_str)) = Self::parse_origin_authority(authority) else {
            return true;
        };

        let is_local = host == "localhost" || host == "127.0.0.1" || host == "[::1]";
        if !is_local {
            return true;
        }

        if !port_str.is_empty() && port_str.parse::<u16>().is_err() {
            return true;
        }

        false
    }

    #[must_use]
    pub fn is_host_header_forbidden(host_header: Option<&str>) -> bool {
        let Some(host_val) = host_header else {
            return false;
        };
        let lower = host_val.trim().to_lowercase();
        if lower.is_empty() {
            return false;
        }
        let host_part = if lower.starts_with('[') {
            if let Some(end_idx) = lower.find(']') {
                &lower[..=end_idx]
            } else {
                return true;
            }
        } else {
            lower.split(':').next().unwrap_or("")
        };
        let is_local = host_part == "localhost" || host_part == "127.0.0.1" || host_part == "[::1]";
        !is_local
    }
}

pub struct EgressFluffStripper;

impl EgressFluffStripper {
    /// Matches a marker only at a word boundary so that technical prose is not
    /// mistaken for filler: "this ensures the invariant holds" must survive even
    /// though it contains the letters of the marker "sure".
    fn contains_marker_at_word_boundary(lower_line: &str, marker: &str) -> bool {
        // Thai is written without inter-word spacing, so a boundary test would
        // reject every legitimate hit; substring matching is correct there.
        if !marker.is_ascii() {
            return lower_line.contains(marker);
        }

        let mut search_offset = 0;
        while let Some(relative_hit) = lower_line[search_offset..].find(marker) {
            let hit_start = search_offset + relative_hit;
            let hit_end = hit_start + marker.len();
            let preceded_by_word = lower_line[..hit_start]
                .chars()
                .next_back()
                .is_some_and(char::is_alphanumeric);
            let followed_by_word = lower_line[hit_end..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric);
            if !preceded_by_word && !followed_by_word {
                return true;
            }
            search_offset = hit_end;
        }
        false
    }

    fn is_pure_conversational_line(line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }
        let lower = trimmed.to_lowercase();
        let fluff_markers = [
            "sure",
            "certainly",
            "here is",
            "here are",
            "below is",
            "here's",
            "i've updated",
            "i have updated",
            "updated code",
            "แน่นอน",
            "ได้ครับ",
            "ได้ค่ะ",
            "โอเค",
            "ต่อไปนี้",
            "นี่คือ",
            "สวัสดี",
        ];
        let has_marker = fluff_markers
            .iter()
            .any(|&fluff| Self::contains_marker_at_word_boundary(&lower, fluff));
        has_marker && trimmed.len() <= 100
    }

    fn drop_leading_fluff_lines(segment: &str) -> (Vec<String>, bool) {
        let mut surviving_lines: Vec<String> = segment.lines().map(str::to_owned).collect();
        let mut stripped_any = false;

        while surviving_lines
            .first()
            .is_some_and(|line| line.trim().is_empty() || Self::is_pure_conversational_line(line))
        {
            surviving_lines.remove(0);
            stripped_any = true;
        }

        (surviving_lines, stripped_any)
    }

    /// Removes an opening filler clause while keeping the substance that follows it
    /// on the same line, so "Certainly, rename the field" yields "rename the field"
    /// instead of being discarded whole.
    fn trim_leading_fluff_clause(line: &str) -> Option<String> {
        let clause_end = line.find(['!', '.', ',', ':', '?'])?;
        let (opening_clause, remainder) = line.split_at(clause_end + 1);
        let remainder = remainder.trim();
        if remainder.is_empty() || !Self::is_pure_conversational_line(opening_clause) {
            return None;
        }
        Some(remainder.to_owned())
    }

    fn purify_prose_head(segment: &str) -> Option<String> {
        let (mut surviving_lines, mut was_modified) = Self::drop_leading_fluff_lines(segment);

        if let Some(first_line) = surviving_lines.first() {
            if let Some(trimmed_clause) = Self::trim_leading_fluff_clause(first_line) {
                surviving_lines[0] = trimmed_clause;
                was_modified = true;
            }
        }
        if !was_modified {
            return None;
        }
        Some(surviving_lines.join("\n"))
    }

    /// Last resort for a reply that is filler end to end: keep the question or
    /// instruction that follows the filler rather than handing back an empty reply.
    fn salvage_substantive_clause(segment: &str) -> Option<String> {
        segment
            .lines()
            .next()
            .and_then(Self::trim_leading_fluff_clause)
    }

    #[must_use]
    pub fn strip_conversational_fluff(raw_stream: &str) -> String {
        let trimmed = raw_stream.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with('#') {
            return raw_stream.to_string();
        }

        // A reply carrying no code fence is still subject to the fluff budget:
        // a bare "Sure! Can you clarify?" must not reach the editor untouched.
        let Some(fence_offset) = trimmed.find("```") else {
            let purified_prose = Self::purify_prose_head(trimmed)
                .map(|purified| purified.trim().to_owned())
                .filter(|purified| !purified.is_empty())
                .or_else(|| Self::salvage_substantive_clause(trimmed));
            return purified_prose.unwrap_or_else(|| raw_stream.to_string());
        };

        let preamble = &trimmed[..fence_offset];
        let code_block = &trimmed[fence_offset..];
        let Some(remaining_preamble) = Self::purify_prose_head(preamble) else {
            return raw_stream.to_string();
        };

        let remaining_preamble = remaining_preamble.trim();
        if remaining_preamble.is_empty() {
            code_block.to_string()
        } else {
            format!("{}\n\n{}", remaining_preamble, code_block)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_english_preamble() {
        let input = "Sure! Here is the updated code:\n\n```rust\nfn main() {}\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(actual, "```rust\nfn main() {}\n```");
    }

    #[test]
    fn test_strip_thai_preamble() {
        let input = "แน่นอนครับ นี่คือโค้ด:\n```csharp\nclass Test {}\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(actual, "```csharp\nclass Test {}\n```");
    }

    #[test]
    fn test_preserve_direct_code_fence() {
        let input = "```typescript\nexport const ok = true;\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(actual, input);
    }

    #[test]
    fn test_strip_fluff_from_reply_without_code_fence() {
        let input = "Sure! Can you provide the code snippet you need to fix?";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(actual, "Can you provide the code snippet you need to fix?");
    }

    #[test]
    fn test_strip_leading_clause_but_keep_substance_on_same_line() {
        let input =
            "Certainly, rename the field to invoice_total and update every call site accordingly.";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(
            actual,
            "rename the field to invoice_total and update every call site accordingly."
        );
    }

    #[test]
    fn test_preserve_prose_containing_marker_inside_a_word() {
        let input = "This ensures the invariant holds.\n\n```rust\nfn main() {}\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(actual, input);
    }

    #[test]
    fn test_preserve_technical_reasoning_while_stripping_greeting() {
        let input = "Sure! Here is the fix:\n\nThe bug happens because the socket was closed prematurely. We updated the connection pool.\n\n```rust\nfn main() {}\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(
            actual,
            "The bug happens because the socket was closed prematurely. We updated the connection pool.\n\n```rust\nfn main() {}\n```"
        );
    }
}
