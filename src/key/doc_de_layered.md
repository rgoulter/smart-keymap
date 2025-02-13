# Layer Modifier Keys

## JSON

```rust
use smart_keymap::key::layered::ModifierKey;

let json = r#"
  { "Hold": 2 }
"#;
let expected_key: ModifierKey = ModifierKey::Hold(2);
let actual_key: ModifierKey = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

# Layered Keys

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered};

type Key = layered::LayeredKey<keyboard::Key>;

let json = r#"
  {
    "base": { "key_code": 4 },
    "layered": [{ "key_code": 5 }, null, { "key_code": 7 }]
  }
"#;
let expected_key: Key = layered::LayeredKey::new(
  keyboard::Key::new(0x04),
  [Some(keyboard::Key::new(0x05)), None, Some(keyboard::Key::new(0x07))],
);
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```
