# JSON

```rust
use smart_keymap::key::keyboard;
use smart_keymap::key::tap_hold::Key;
let json = r#"
  { "hold": { "key_code": 224 }, "tap": { "key_code": 4 } }
"#;
let expected_key: Key<keyboard::Key> = Key {
  hold: keyboard::Key::new(224),
  tap: keyboard::Key::new(4),
};
let actual_key: Key<keyboard::Key> = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

