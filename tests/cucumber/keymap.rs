use std::fmt::Debug;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use serde::Deserialize;

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;

use smart_keymap_nickel_helper::{
    nickel_json_serialization_for_keymap, nickel_to_json_for_hid_report, NickelError,
};

type Key = key::composite::Key;
type Context = key::composite::Context;
type Event = key::composite::Event;

type DynamicKey = key::dynamic::DynamicKey<Key, Context, Event>;

#[derive(Debug)]
enum LoadedKeymap {
    NoKeymap,
    Keymap(keymap::Keymap<Vec<DynamicKey>>),
}

impl LoadedKeymap {
    pub fn handle_input(&mut self, ev: input::Event) {
        match self {
            LoadedKeymap::Keymap(keymap) => keymap.handle_input(ev),
            _ => panic!("No keymap loaded"),
        }
    }
    pub fn tick(&mut self) {
        match self {
            LoadedKeymap::Keymap(keymap) => keymap.tick(),
            _ => panic!("No keymap loaded"),
        }
    }
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        match self {
            LoadedKeymap::Keymap(keymap) => {
                smart_keymap::keymap::KeymapOutput::new(keymap.pressed_keys())
                    .as_hid_boot_keyboard_report()
            }
            _ => panic!("No keymap loaded"),
        }
    }
}
#[derive(Debug, World)]
pub struct KeymapWorld {
    keymap: LoadedKeymap,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        KeymapWorld {
            keymap: LoadedKeymap::NoKeymap,
        }
    }
}

#[derive(Deserialize)]
struct Keymap {
    config: key::composite::Config,
    keys: Vec<Key>,
}

#[given("a keymap.ncl:")]
fn setup_nickel_keymap(world: &mut KeymapWorld, step: &Step) {
    let keymap_ncl = step.docstring().unwrap();
    match nickel_json_serialization_for_keymap(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        keymap_ncl,
    ) {
        Ok(json) => {
            let keymap_result: serde_json::Result<Keymap> = serde_json::from_str(&json);
            match keymap_result {
                Ok(keymap) => {
                    let dyn_keys = keymap
                        .keys
                        .into_iter()
                        .map(|k| DynamicKey::new(k))
                        .collect();
                    let context = key::composite::Context::from_config(keymap.config);
                    world.keymap = LoadedKeymap::Keymap(keymap::Keymap::new(dyn_keys, context));
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
