# Layer Modifier Keys

## JSON

```rust
use smart_keymap::key::layered::ModifierKey;

let json = r#"
  { "Hold": 2 }
"#;
let expected_key: ModifierKey = ModifierKey::Hold(2);
let actual_key: ModifierKey = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# Layered Keys

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered, simple};

type L = layered::ArrayImpl<3>;
type Key = layered::LayeredKey<simple::Key, L>;

let json = r#"
  {
    "base": 4,
    "layered": [5, null, 7]
  }
"#;
let expected_key: Key = layered::LayeredKey {
  base: simple::Key(0x04),
  layered: [Some(simple::Key(0x05)), None, Some(simple::Key(0x07))],
};
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
