use std::fmt::Debug;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::Keymap;
use smart_keymap::tuples;

mod common;

use common::Deserializer;

/// Evaluates the Nickel expr for a keymap, returning the json serialization.
fn nickel_json_serialization_for_keymap(keymap_ncl: &str) -> io::Result<String> {
    let mut nickel_command = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={}/ncl", env!("CARGO_MANIFEST_DIR")).as_ref(),
            "--field=serialized_json_composite_keys",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = nickel_command.stdin.as_mut().unwrap();
    child_stdin
        .write_all(format!(r#"(import "keymap-ncl-to-json.ncl") & ({})"#, keymap_ncl).as_bytes())?;

    let output = nickel_command.wait_with_output()?;

    String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Evaluates the Nickel expr for an HID, returning the json serialization.
fn nickel_to_json_for_hid_report(keymap_ncl: &str) -> io::Result<String> {
    let mut nickel_command = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={}/ncl", env!("CARGO_MANIFEST_DIR")).as_ref(),
            "--field=as_bytes",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = nickel_command.stdin.as_mut().unwrap();
    child_stdin.write_all(format!(r#"(import "hid-report.ncl") & ({})"#, keymap_ncl).as_bytes())?;

    let output = nickel_command.wait_with_output()?;

    String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

type NestedKey = key::composite::DefaultNestableKey;
type LayersImpl = key::layered::ArrayImpl<16>;
type Key = key::composite::Key<NestedKey, LayersImpl>;
type Context = key::composite::Context<LayersImpl>;
type Event = key::composite::Event;

#[derive(Debug)]
enum LoadedKeymap {
    NoKeymap,
    Keymap1(Keymap<tuples::Keys1<Key, Context, Event>, LayersImpl>),
    Keymap2(Keymap<tuples::Keys2<Key, Key, Context, Event>, LayersImpl>),
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
                key::composite::Context::default(),
            )),
            2 => LoadedKeymap::Keymap2(Keymap::new(
                tuples::Keys2::new((keys[0], keys[1])),
                key::composite::Context::default(),
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
    match nickel_json_serialization_for_keymap(keymap_ncl) {
        Ok(json) => {
            let keys_vec: Vec<Key> = Deserializer::JSON.from_str(&json).unwrap();
            world.keymap = keys_vec.into();
        }
        Err(e) => panic!("Failed to convert keymap.ncl to json: {:?}", e),
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
    match nickel_to_json_for_hid_report(hid_report_ncl) {
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
    match nickel_to_json_for_hid_report(hid_report_ncl) {
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
