# Simple variant

# JSON

```rust
use smart_keymap::key;

use key::{composite, layered, simple};

use composite::DefaultNestableKey;

type L = layered::ArrayImpl<1>;
type Ctx = composite::Context<DefaultNestableKey, L>;
type Key = composite::Key<DefaultNestableKey, L>;

let json = r#"
  { "Simple": { "key": 4 } }
"#;
let expected_key: Key = composite::Key::Simple { key: simple::Key(0x04) };
let actual_key: Key = serde_json::from_str(json).unwrap();
assert_eq!(actual_key, expected_key);
```
