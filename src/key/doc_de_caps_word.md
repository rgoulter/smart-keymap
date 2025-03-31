# JSON

Plain key code:

```rust
use smart_keymap::key::caps_word::Key;
let json = r#"
  "ToggleCapsWord"
"#;
let expected_key: Key = Key::ToggleCapsWord;
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

