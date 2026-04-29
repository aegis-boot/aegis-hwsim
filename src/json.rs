//! Minimal JSON string escaper, shared across the modules that emit
//! `schema_version=1` JSON envelopes (`bin/aegis-hwsim`, `coverage_grid`,
//! `doctor`).
//!
//! Why a hand-rolled escaper rather than `serde_json` for everything:
//! the JSON output we produce is small, fixed-shape, and built by
//! formatting whole-document templates rather than driving serde's
//! `Serializer`. Pulling `serde_json::to_string_pretty` for one-line
//! string escapes adds an allocation + a serializer instantiation per
//! escape; the hand-rolled version is a single pass over the input.
//!
//! The function is deliberately conservative: it escapes the JSON
//! grammar's mandatory escapes (`"`, `\`), the four common
//! whitespace-control chars (`\n`, `\r`, `\t`, plus the implicit
//! sub-`0x20` `\uXXXX` form), and nothing else. UTF-8 is passed through
//! verbatim — same shape `serde_json` produces by default.
//!
//! Until #58 there were three near-identical copies of this function;
//! a comment in `doctor.rs` documented the policy as "extract on the
//! fourth caller". The fourth caller (`test_keyring`'s README emitter)
//! never quite happened, but the duplication was already a hygiene
//! debt the code review flagged. This module closes it.

/// Escape `s` for inclusion as a JSON string body (no surrounding quotes).
///
/// Escapes:
/// - `"`  → `\"`
/// - `\`  → `\\`
/// - `\n` → `\n`
/// - `\r` → `\r`
/// - `\t` → `\t`
/// - any other control char (`< 0x20`) → `\u00XX`
///
/// All other characters (including non-ASCII UTF-8) pass through unchanged.
#[must_use]
pub fn escape(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn escape_passes_ascii_unchanged() {
        assert_eq!(escape("hello world"), "hello world");
        assert_eq!(escape(""), "");
    }

    #[test]
    fn escape_doubles_backslash() {
        assert_eq!(escape(r"path\to\file"), r"path\\to\\file");
    }

    #[test]
    fn escape_escapes_double_quote() {
        assert_eq!(escape("say \"hi\""), "say \\\"hi\\\"");
    }

    #[test]
    fn escape_handles_common_whitespace_controls() {
        assert_eq!(escape("a\nb"), "a\\nb");
        assert_eq!(escape("a\rb"), "a\\rb");
        assert_eq!(escape("a\tb"), "a\\tb");
    }

    #[test]
    fn escape_emits_uxxxx_for_other_control_chars() {
        // 0x01 (SOH), 0x07 (BEL), 0x1f (US): all get \u00XX form.
        assert_eq!(escape("\x01"), "\\u0001");
        assert_eq!(escape("\x07"), "\\u0007");
        assert_eq!(escape("\x1f"), "\\u001f");
    }

    #[test]
    fn escape_passes_non_ascii_utf8_unchanged() {
        // Greek + Chinese + emoji — all pass through as raw UTF-8,
        // matching serde_json's default (it doesn't \uXXXX-escape
        // BMP/non-BMP unless asked).
        assert_eq!(escape("ναί"), "ναί");
        assert_eq!(escape("好"), "好");
        assert_eq!(escape("🦀"), "🦀");
    }

    #[test]
    fn escape_output_round_trips_through_serde_json() {
        // The whole point of this function is to produce something
        // that can be wrapped in `"..."` and parsed as valid JSON.
        // Round-trip a torture string through serde_json::from_str
        // to confirm the escape contract holds.
        let input = "quote\" backslash\\ newline\n tab\t ctrl\x01 utf8\u{1F980} end";
        let json_body = format!("\"{}\"", escape(input));
        let parsed: String = serde_json::from_str(&json_body).expect("must parse");
        assert_eq!(parsed, input);
    }
}
