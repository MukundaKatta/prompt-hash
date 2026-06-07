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

#[test]
fn body_cannot_be_confused_with_extra_messages() {
    // A single message whose body embeds the field markers must NOT collide
    // with two distinct messages. (Regression test for delimiter injection.)
    let single = key("m", &[("user", "a\nrole=user\nbody=b")], 1.0);
    let two = key("m", &[("user", "a"), ("user", "b")], 1.0);
    assert_ne!(single, two);
}

#[test]
fn role_cannot_be_confused_with_body() {
    // Shifting characters across the role/body boundary must change the key.
    let a = key("m", &[("userbody", "x")], 1.0);
    let b = key("m", &[("user", "bodyx")], 1.0);
    assert_ne!(a, b);
}

#[test]
fn message_count_matters() {
    let one = key("m", &[("user", "")], 1.0);
    let two = key("m", &[("user", ""), ("user", "")], 1.0);
    assert_ne!(one, two);
}

#[test]
fn identical_inputs_are_stable() {
    let a = key("claude", &[("user", "hello"), ("assistant", "hi")], 0.7);
    let b = key("claude", &[("user", "hello"), ("assistant", "hi")], 0.7);
    assert_eq!(a, b);
}
