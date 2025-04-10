use std::mem;

use serde::Serialize;

use smart_keymap::{input, key};

#[derive(Serialize)]
struct StructSizes {
    input_event: usize,
    key_keyboardmodifiers: usize,
    key_composite_context: usize,
    key_composite_event: usize,
    key_composite_key: usize,
    key_composite_pendingkeystate: usize,
    key_composite_keystate: usize,
    keymap_keymap: usize,
}

fn main() {
    let sizes_data = StructSizes {
        input_event: mem::size_of::<input::Event>(),
        key_keyboardmodifiers: mem::size_of::<key::KeyboardModifiers>(),
        key_composite_context: mem::size_of::<key::composite::Context>(),
        key_composite_event: mem::size_of::<key::composite::Event>(),
        key_composite_key: mem::size_of::<key::composite::Key>(),
        key_composite_pendingkeystate: mem::size_of::<key::composite::PendingKeyState>(),
        key_composite_keystate: mem::size_of::<key::composite::KeyState>(),
        keymap_keymap: mem::size_of::<smart_keymap::init::Keymap>(),
    };

    let json_output = serde_json::to_string_pretty(&sizes_data).unwrap();
    println!("{}", json_output);
}
