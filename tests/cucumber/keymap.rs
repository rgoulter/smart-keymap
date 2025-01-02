use std::fmt::Debug;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::Keymap;
use smart_keymap::tuples;

use key::composite::Key;

mod common;

use common::Deserializer;

#[derive(Debug)]
enum LoadedKeymap {
    NoKeymap,
    Keymap1(Keymap<tuples::Keys1<Key>>),
    Keymap2(Keymap<tuples::Keys2<Key, Key>>),
}

impl LoadedKeymap {
    pub fn handle_input(&mut self, ev: input::Event) {
        match self {
            LoadedKeymap::Keymap1(keymap) => keymap.handle_input(ev),
            LoadedKeymap::Keymap2(keymap) => keymap.handle_input(ev),
            _ => panic!("No keymap loaded"),
        }
    }
    pub fn tick(&mut self) {
        match self {
            LoadedKeymap::Keymap1(keymap) => keymap.tick(),
            LoadedKeymap::Keymap2(keymap) => keymap.tick(),
            _ => panic!("No keymap loaded"),
        }
    }
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        match self {
            LoadedKeymap::Keymap1(keymap) => keymap.boot_keyboard_report(),
            LoadedKeymap::Keymap2(keymap) => keymap.boot_keyboard_report(),
            _ => panic!("No keymap loaded"),
        }
    }
}

impl From<Vec<Key>> for LoadedKeymap {
    fn from(keys: Vec<Key>) -> Self {
        match keys.len() {
            1 => LoadedKeymap::Keymap1(Keymap::new(
                tuples::Keys1::new((keys[0],)),
                key::composite::Context::new(),
            )),
            2 => LoadedKeymap::Keymap2(Keymap::new(
                tuples::Keys2::new((keys[0], keys[1])),
                key::composite::Context::new(),
            )),
            _ => panic!("Cucumber impl doesn't support Keys{}", keys.len()),
        }
    }
}

#[derive(Debug, World)]
pub struct KeymapWorld {
    input_deserializer: Deserializer,
    keymap: LoadedKeymap,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        KeymapWorld {
            input_deserializer: Deserializer::JSON,
            keymap: LoadedKeymap::NoKeymap,
        }
    }
}

#[given(expr = "a keymap, expressed as a {deserializer} string")]
fn setup_keymap(world: &mut KeymapWorld, step: &Step, deserializer: Deserializer) {
    let keys_vec: Vec<Key> = deserializer
        .from_str(step.docstring().as_ref().unwrap())
        .unwrap();

    world.input_deserializer = deserializer;
    world.keymap = keys_vec.into();
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
