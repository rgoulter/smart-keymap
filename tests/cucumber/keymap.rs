use std::fmt::Debug;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::Keymap;

use key::composite::Key;

mod common;

use common::Deserializer;

#[derive(Debug, World)]
pub struct KeymapWorld {
    input_deserializer: Deserializer,
    keymap: Keymap<Vec<Key>>,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        let keymap = Keymap::new(Vec::new(), key::composite::Context::new());
        KeymapWorld {
            input_deserializer: Deserializer::JSON,
            keymap,
        }
    }
}

#[given(expr = "a keymap, expressed as a {deserializer} string")]
fn setup_keymap(world: &mut KeymapWorld, step: &Step, deserializer: Deserializer) {
    let keys: Vec<Key> = deserializer
        .from_str(step.docstring().as_ref().unwrap())
        .unwrap();

    world.input_deserializer = deserializer;
    world.keymap = Keymap::new(keys, key::composite::Context::new());
}

#[when("the keymap registers the following input")]
fn perform_input(world: &mut KeymapWorld, step: &Step) {
    let inputs: Vec<input::Event> = world
        .input_deserializer
        .from_str(step.docstring().as_ref().unwrap())
        .unwrap();

    for input in inputs {
        world.keymap.handle_input(input);
    }
}

#[when(expr = "the keymap ticks {int} times")]
fn when_keymap_tick(world: &mut KeymapWorld, num_ticks: u16) {
    for _ in 0..num_ticks {
        world.keymap.tick();
    }
}

#[then("the HID keyboard report should be")]
fn check_report(world: &mut KeymapWorld, step: &Step) {
    let expected_report: Vec<u8> = world
        .input_deserializer
        .from_str(step.docstring().as_ref().unwrap())
        .unwrap();

    let actual_report = world.keymap.boot_keyboard_report();

    assert_eq!(expected_report, actual_report);
}

#[then("the HID keyboard report from the next tick() should be")]
fn check_tick_report(world: &mut KeymapWorld, step: &Step) {
    let expected_report: Vec<u8> = world
        .input_deserializer
        .from_str(step.docstring().as_ref().unwrap())
        .unwrap();

    world.keymap.tick();
    let actual_report = world.keymap.boot_keyboard_report();

    assert_eq!(expected_report, actual_report);
}

fn main() {
    futures::executor::block_on(KeymapWorld::run("features/keymap/"));
}
