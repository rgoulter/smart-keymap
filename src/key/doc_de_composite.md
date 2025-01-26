# Keyboard variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type T = composite::CompositeImpl<NK>;
type Ctx = composite::Context;
type Key = composite::Key<T>;

let json = r#"
  { "Keyboard": { "key_code": 4 } }
"#;
let expected_key: Key = composite::Key::Keyboard(keyboard::Key::new(0x04));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# TapHold variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered, tap_hold};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type T = composite::CompositeImpl<NK>;
type Ctx = composite::Context;
type Key = composite::Key<T>;

let json = r#"
  { "TapHold": { "hold": { "key_code": 224 }, "tap": { "key_code": 4 } } }
"#;
let expected_key: Key = composite::Key::TapHold(tap_hold::Key {
    tap: keyboard::Key::new(4),
    hold: keyboard::Key::new(224),
  });
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
type T = composite::CompositeImpl<NK>;
type Ctx = composite::Context;
type Key = composite::Key<T>;

let json = r#"
  { "LayerModifier": { "Hold": 2 } }
"#;
let expected_key: Key = composite::Key::LayerModifier(layered::ModifierKey::Hold(2));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```

# Layered Key variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered};

use composite::DefaultNestableKey;

type NK = composite::DefaultNestableKey;
type T = composite::CompositeImpl<NK>;
type Ctx = composite::Context;
type Key = composite::Key<T>;

let json = r#"
  {
    "Layered": {
      "base": { "key_code": 4 },
      "layered": [{ "key_code": 5 }, null, { "key_code": 7 }]
    }
  }
"#;
let expected_key: Key = composite::Key::Layered(layered::LayeredKey::new(
    keyboard::Key::new(0x04),
    [Some(keyboard::Key::new(0x05)), None, Some(keyboard::Key::new(0x07))],
  ));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
