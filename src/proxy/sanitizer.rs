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
            .any(|&fluff| lower.starts_with(fluff) || lower.contains(fluff));
        has_marker && trimmed.len() <= 100
    }

    #[must_use]
    pub fn strip_conversational_fluff(raw_stream: &str) -> String {
        let trimmed = raw_stream.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with('#') {
            return raw_stream.to_string();
        }

        let Some(pos) = trimmed.find("```") else {
            return raw_stream.to_string();
        };

        let preamble = &trimmed[..pos];
        let code_block = &trimmed[pos..];

        let mut lines: Vec<&str> = preamble.lines().collect();
        let mut stripped_any = false;

        while let Some(first_line) = lines.first() {
            if first_line.trim().is_empty() || Self::is_pure_conversational_line(first_line) {
                lines.remove(0);
                stripped_any = true;
            } else {
                break;
            }
        }

        if !stripped_any {
            return raw_stream.to_string();
        }

        let remaining_preamble = lines.join("\n").trim().to_string();
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
    fn test_preserve_technical_reasoning_while_stripping_greeting() {
        let input = "Sure! Here is the fix:\n\nThe bug happens because the socket was closed prematurely. We updated the connection pool.\n\n```rust\nfn main() {}\n```";
        let actual = EgressFluffStripper::strip_conversational_fluff(input);
        assert_eq!(
            actual,
            "The bug happens because the socket was closed prematurely. We updated the connection pool.\n\n```rust\nfn main() {}\n```"
        );
    }
}
