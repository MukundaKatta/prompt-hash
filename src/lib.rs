//! # prompt-hash
//!
//! Deterministic cache key for an LLM prompt.
//!
//! Inputs:
//! - `model` — model id (already normalized by `claude-cost`/`openai-cost`
//!   if you care).
//! - `messages` — `[(role, content)]`.
//! - `temperature` — quantized to 2 decimal places to avoid floating
//!   point cache misses.
//!
//! Output: 64-char hex SHA-256.
//!
//! Use as a request-level cache key. For semantic cache keys (matching
//! prompts with the same meaning but different wording), pair with
//! [`semantic-cache-key`](https://crates.io/crates/semantic-cache-key).
//!
//! ## Example
//!
//! ```
//! use prompt_hash::key;
//! let k1 = key(
//!     "claude-sonnet-4-5",
//!     &[("user", "what is 2+2?")],
//!     1.0,
//! );
//! let k2 = key(
//!     "claude-sonnet-4-5",
//!     &[("user", "what is 2+2?   ")], // trailing ws ignored
//!     1.0,
//! );
//! assert_eq!(k1, k2);
//! assert_eq!(k1.len(), 64);
//! ```

#![deny(missing_docs)]

mod sha256;

/// Compute a 64-char hex cache key.
pub fn key(model: &str, messages: &[(&str, &str)], temperature: f32) -> String {
    let mut input = String::new();
    input.push_str("model=");
    input.push_str(model);
    input.push('\n');
    input.push_str("temp=");
    // Quantize to 2 decimals (1.0 == 1.0000001).
    input.push_str(&format!("{:.2}", temperature));
    input.push('\n');
    for (role, content) in messages {
        input.push_str("role=");
        input.push_str(role);
        input.push('\n');
        input.push_str("body=");
        input.push_str(&normalize_body(content));
        input.push('\n');
    }
    sha256::hex(input.as_bytes())
}

fn normalize_body(s: &str) -> String {
    // Normalize CRLF -> LF, strip trailing whitespace per line.
    let s = s.replace("\r\n", "\n").replace('\r', "\n");
    s.lines()
        .map(|l| l.trim_end_matches(|c: char| c == ' ' || c == '\t'))
        .collect::<Vec<_>>()
        .join("\n")
}
