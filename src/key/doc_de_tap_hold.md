# JSON

```rust
use smart_keymap::key::tap_hold::Key;
let json = r#"
  { "hold": 224, "tap": 4 }
"#;
let expected_key: Key = Key { hold: 224, tap: 4 };
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

