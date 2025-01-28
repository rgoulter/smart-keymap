use std::fmt::Debug;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::Keymap;
use smart_keymap::tuples;

mod common;

use common::Deserializer;

use smart_keymap_nickel_helper::{
    nickel_json_serialization_for_keymap, nickel_to_json_for_hid_report, NickelError,
};

type Key = key::composite::Key;
type Context = key::composite::Context;
type Event = key::composite::Event;

#[derive(Debug)]
enum LoadedKeymap {
    NoKeymap,
    Keymap1(Keymap<tuples::Keys1<Key, Context, Event>>),
    Keymap2(Keymap<tuples::Keys2<Key, Key, Context, Event>>),
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
            LoadedKeymap::Keymap1(keymap) => {
                smart_keymap::keymap::KeymapOutput::new(keymap.pressed_keys())
                    .as_hid_boot_keyboard_report()
            }
            LoadedKeymap::Keymap2(keymap) => {
                smart_keymap::keymap::KeymapOutput::new(keymap.pressed_keys())
                    .as_hid_boot_keyboard_report()
            }
            _ => panic!("No keymap loaded"),
        }
    }
}

impl From<Vec<Key>> for LoadedKeymap {
    fn from(keys: Vec<Key>) -> Self {
        match keys.len() {
            1 => LoadedKeymap::Keymap1(Keymap::new(
                tuples::Keys1::new((keys[0],)),
                key::composite::DEFAULT_CONTEXT,
            )),
            2 => LoadedKeymap::Keymap2(Keymap::new(
                tuples::Keys2::new((keys[0], keys[1])),
                key::composite::DEFAULT_CONTEXT,
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
            input_deserializer: Deserializer::RON,
            keymap: LoadedKeymap::NoKeymap,
        }
    }
}

#[given("a keymap.ncl:")]
fn setup_nickel_keymap(world: &mut KeymapWorld, step: &Step) {
    let keymap_ncl = step.docstring().unwrap();
    match nickel_json_serialization_for_keymap(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        keymap_ncl,
    ) {
        Ok(json) => {
            let keys_vec_result: serde_json::Result<Vec<Key>> = serde_json::from_str(&json);
            match keys_vec_result {
                Ok(keys_vec) => {
                    world.keymap = keys_vec.into();
                }
                Err(e) => {
                    panic!(
                        "\n\nerror deserailizing JSON:\n\nDeserialization Error:\n\n{}\n\nJSON:\n{}",
                        e,
                        json,
                    )
                }
            }
        }
        Err(e) => match e {
            NickelError::NickelNotFound => panic!("`nickel` not found on PATH. Please install it."),
            NickelError::EvalError(nickel_error_message) => panic!(
                "\n\nerror evaluating step's doc string nickel:\n\n{}",
                nickel_error_message
            ),
        },
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
    let inputs: Vec<input::Event> = ron::from_str(step.docstring().as_ref().unwrap()).unwrap();

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

#[then("the HID keyboard report should equal")]
fn check_report(world: &mut KeymapWorld, step: &Step) {
    let hid_report_ncl = step.docstring().unwrap();
    match nickel_to_json_for_hid_report(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        hid_report_ncl,
    ) {
        Ok(json) => {
            let expected_report: Vec<u8> = serde_json::from_str(&json).unwrap();

            let actual_report = world.keymap.boot_keyboard_report();

            assert_eq!(expected_report, actual_report);
        }
        Err(e) => panic!("Failed to convert keymap.ncl to json: {:?}", e),
    }
}

#[then("the HID keyboard report from the next tick() should equal")]
fn check_tick_report(world: &mut KeymapWorld, step: &Step) {
    let hid_report_ncl = step.docstring().unwrap();
    match nickel_to_json_for_hid_report(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        hid_report_ncl,
    ) {
        Ok(json) => {
            let expected_report: Vec<u8> = serde_json::from_str(&json).unwrap();

            world.keymap.tick();
            let actual_report = world.keymap.boot_keyboard_report();

            assert_eq!(expected_report, actual_report);
        }
        Err(e) => panic!("Failed to convert keymap.ncl to json: {:?}", e),
    }
}

fn main() {
    futures::executor::block_on(KeymapWorld::run("features/keymap/"));
}
