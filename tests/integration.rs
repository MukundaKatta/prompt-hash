use prompt_hash::key;

#[test]
fn whitespace_doesnt_change_key() {
    let a = key("claude", &[("user", "hi   ")], 1.0);
    let b = key("claude", &[("user", "hi")], 1.0);
    assert_eq!(a, b);
}

#[test]
fn temperature_change_changes_key() {
    let a = key("claude", &[("user", "x")], 0.5);
    let b = key("claude", &[("user", "x")], 0.7);
    assert_ne!(a, b);
}

#[test]
fn temperature_within_2_decimals_is_same() {
    let a = key("claude", &[("user", "x")], 0.5);
    let b = key("claude", &[("user", "x")], 0.501);
    assert_eq!(a, b);
}

#[test]
fn model_change_changes_key() {
    let a = key("claude", &[("user", "x")], 1.0);
    let b = key("gpt-5", &[("user", "x")], 1.0);
    assert_ne!(a, b);
}

#[test]
fn role_order_matters() {
    let a = key("c", &[("user", "1"), ("assistant", "2")], 1.0);
    let b = key("c", &[("assistant", "2"), ("user", "1")], 1.0);
    assert_ne!(a, b);
}

#[test]
fn returns_64_hex() {
    let k = key("c", &[("user", "x")], 1.0);
    assert_eq!(k.len(), 64);
    assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
}
