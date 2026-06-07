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
    push_field(&mut input, "model", model);
    // Quantize to 2 decimals (1.0 == 1.0000001).
    push_field(&mut input, "temp", &format!("{temperature:.2}"));
    // Frame the message count so a different number of messages can never
    // be confused with message contents that happen to contain digits.
    push_field(&mut input, "count", &messages.len().to_string());
    for (role, content) in messages {
        push_field(&mut input, "role", role);
        push_field(&mut input, "body", &normalize_body(content));
    }
    sha256::hex(input.as_bytes())
}

/// Append a length-prefixed field to `out`.
///
/// Serializing as `name=<byte-len>:<value>` makes the encoding injective:
/// because every value is preceded by its exact byte length, no value can
/// ever be mistaken for a delimiter or another field. This prevents
/// collisions where, e.g., a single message whose body embeds `role=`/`body=`
/// markers would otherwise hash the same as two separate messages.
fn push_field(out: &mut String, name: &str, value: &str) {
    out.push_str(name);
    out.push('=');
    out.push_str(&value.len().to_string());
    out.push(':');
    out.push_str(value);
    out.push('\n');
}

fn normalize_body(s: &str) -> String {
    // Normalize CRLF -> LF, strip trailing whitespace per line.
    let s = s.replace("\r\n", "\n").replace('\r', "\n");
    s.lines()
        .map(|l| l.trim_end_matches([' ', '\t']))
        .collect::<Vec<_>>()
        .join("\n")
}
