# JSON

```rust
use smart_keymap::key::simple;
use smart_keymap::key::tap_hold::Key;
let json = r#"
  { "hold": 224, "tap": 4 }
"#;
let expected_key: Key<simple::Key> = Key {
  hold: simple::Key(224),
  tap: simple::Key(4),
};
let actual_key: Key<simple::Key> = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

