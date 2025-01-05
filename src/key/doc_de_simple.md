# JSON

```rust
use smart_keymap::key::simple::Key;
let json = r#"
  4
"#;
let expected_key: Key = Key(0x04);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
