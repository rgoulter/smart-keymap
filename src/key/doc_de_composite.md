# Keyboard variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard};

type Ctx = composite::Context;
type Key = composite::BaseKey;

let json = r#"
  { "key_code": 4 }
"#;
let expected_key: Key = Key::Keyboard(keyboard::Key::new(0x04));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

# TapHold variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered, tap_hold};

type Ctx = composite::Context;
type Key = composite::TapHoldKey<composite::BaseKey>;

let json = r#"
  { "hold": { "key_code": 224 }, "tap": { "key_code": 4 } }
"#;
let expected_key: Key = Key::tap_hold(tap_hold::Key {
    tap: keyboard::Key::new(4).into(),
    hold: keyboard::Key::new(224).into(),
  });
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

# Layer Modifier Key variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, layered};

type Ctx = composite::Context;
type Key = composite::BaseKey;

let json = r#"
  { "Hold": 2 }
"#;
let expected_key: Key = Key::LayerModifier(layered::ModifierKey::Hold(2));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```

# Layered Key variant

## JSON

```rust
use smart_keymap::key;

use key::{composite, keyboard, layered};

type Ctx = composite::Context;
type Key = composite::LayeredKey<composite::TapHoldKey<composite::BaseKey>>;

let json = r#"
  {
    "base": { "key_code": 4 },
    "layered": [{ "key_code": 5 }, null, { "key_code": 7 }]
  }
"#;
let expected_key: Key = Key::layered(layered::LayeredKey::new(
    keyboard::Key::new(0x04).into(),
    [Some(keyboard::Key::new(0x05).into()), None, Some(keyboard::Key::new(0x07).into())],
  ));
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(expected_key, actual_key);
```
