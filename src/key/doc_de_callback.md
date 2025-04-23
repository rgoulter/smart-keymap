# JSON

"Reset to bootloader" callback key:

```rust
use smart_keymap::key::callback::Key;
use smart_keymap::keymap::KeymapCallback;
let json = r#"
  {"keymap_callback": "ResetToBootloader"}
"#;
let expected_key: Key = Key::new(KeymapCallback::ResetToBootloader);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

Custom callback key:

```rust
use smart_keymap::key::callback::Key;
use smart_keymap::keymap::KeymapCallback;
let json = r#"
  {"keymap_callback": { "Custom": [3, 4] }}
"#;
let expected_key: Key = Key::new(KeymapCallback::Custom(3, 4));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```
