# JSON

```rust
use smart_keymap::key::keyboard;
use smart_keymap::key::tap_hold::Key;
let json = r#"
  { "hold": 224, "tap": 4 }
"#;
let expected_key: Key<keyboard::Key> = Key {
  hold: keyboard::Key(224),
  tap: keyboard::Key(4),
};
let actual_key: Key<keyboard::Key> = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

