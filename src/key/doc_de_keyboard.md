# JSON

```rust
use smart_keymap::key::keyboard::Key;
let json = r#"
  {"key_code": 4}
"#;
let expected_key: Key = Key::new(0x04);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
