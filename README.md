# prompt-hash

[![crates.io](https://img.shields.io/crates/v/prompt-hash.svg)](https://crates.io/crates/prompt-hash)

Deterministic 64-char cache key for an LLM prompt. Whitespace-stable,
temperature quantized to 2 decimals.

```rust
use prompt_hash::key;
let k = key("claude-sonnet-4-5", &[("user", "what is 2+2?")], 1.0);
```

Zero deps. MIT or Apache-2.0.
