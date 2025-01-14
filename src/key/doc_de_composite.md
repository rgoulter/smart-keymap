# Simple variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered, simple};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type L = layered::ArrayImpl<1>;
type T = composite::CompositeImpl<NK, L>;
type Ctx = composite::Context<T>;
type Key = composite::Key<T>;

let json = r#"
  { "Simple": { "key": 4 } }
"#;
let expected_key: Key = composite::Key::Simple { key: simple::Key(0x04) };
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# TapHold variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered, simple, tap_hold};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type L = layered::ArrayImpl<1>;
type T = composite::CompositeImpl<NK, L>;
type Ctx = composite::Context<T>;
type Key = composite::Key<T>;

let json = r#"
  { "TapHold": { "key": { "hold": 224, "tap": 4 } } }
"#;
let expected_key: Key = composite::Key::TapHold {
  key: tap_hold::Key {
    tap: simple::Key(4),
    hold: simple::Key(224),
  },
};
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# Layer Modifier Key variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type L = layered::ArrayImpl<1>;
type T = composite::CompositeImpl<NK, L>;
type Ctx = composite::Context<T>;
type Key = composite::Key<T>;

let json = r#"
  { "LayerModifier": { "key": { "Hold": 2 } } }
"#;
let expected_key: Key = composite::Key::LayerModifier { key: layered::ModifierKey::Hold(2) };
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# Layered Key variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered, simple};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type L = layered::ArrayImpl<3>;
type T = composite::CompositeImpl<NK, L>;
type Ctx = composite::Context<T>;
type Key = composite::Key<T>;

let json = r#"
  {
    "Layered": {
      "key": {
        "base": 4,
        "layered": [5, null, 7]
      }
    }
  }
"#;
let expected_key: Key = composite::Key::Layered {
  key: layered::LayeredKey {
    base: simple::Key(0x04),
    layered: [Some(simple::Key(0x05)), None, Some(simple::Key(0x07))],
  }
};
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
