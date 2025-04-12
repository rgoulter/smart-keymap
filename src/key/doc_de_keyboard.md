# JSON

Plain key code:

```rust
use smart_keymap::key::keyboard::Key;
let json = r#"
  {"key_code": 4}
"#;
let expected_key: Key = Key::new(0x04);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

Modifiers:

```rust
use smart_keymap::key::keyboard::Key;
use smart_keymap::key::KeyboardModifiers;
let json = r#"
  { "modifiers": 1 }
"#;
let expected_key: Key = Key::from_modifiers(
    KeyboardModifiers::LEFT_CTRL,
);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

Key code with modifiers:

```rust
use smart_keymap::key::keyboard::Key;
use smart_keymap::key::KeyboardModifiers;
let json = r#"
  { "key_code": 4, "modifiers": 1 }
"#;
let expected_key: Key = Key::new_with_modifiers(
    0x04,
    KeyboardModifiers::LEFT_CTRL,
);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```
