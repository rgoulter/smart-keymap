use std::fmt::Debug;

use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use serde::Deserialize;

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;

use smart_keymap_nickel_helper::{
    nickel_json_serialization_for_inputs, nickel_json_serialization_for_keymap,
    nickel_to_json_for_hid_report, NickelError,
};

type Key = key::composite::Key;

type Keymap = keymap::Keymap<Vec<Key>>;

/// Keymap with basic keycodes, useful for the "check report equivalences" step.
const TEST_KEYMAP_NCL: &str = r#"
  let K = import "keys.ncl" in
  { keys = [ K.A, K.B, K.C, K.LeftCtrl ] }
"#;

#[derive(Debug)]
enum LoadedKeymap {
    NoKeymap,
    Keymap {
        keymap: Keymap,
        distinct_reports: keymap::DistinctReports,
    },
}

impl LoadedKeymap {
    pub fn keymap(keymap: Keymap) -> Self {
        LoadedKeymap::Keymap {
            keymap,
            distinct_reports: keymap::DistinctReports::new(),
        }
    }

    pub fn handle_input(&mut self, ev: input::Event) {
        match self {
            LoadedKeymap::Keymap {
                keymap,
                distinct_reports,
            } => {
                keymap.handle_input(ev);
                distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

                for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
                    keymap.tick();
                    distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
                }
            }
            _ => panic!("No keymap loaded"),
        }
    }

    pub fn tick(&mut self) {
        match self {
            LoadedKeymap::Keymap {
                keymap,
                distinct_reports,
            } => {
                keymap.tick();
                distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
            }
            _ => panic!("No keymap loaded"),
        }
    }

    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        match self {
            LoadedKeymap::Keymap { keymap, .. } => {
                smart_keymap::keymap::KeymapOutput::new(keymap.pressed_keys())
                    .as_hid_boot_keyboard_report()
            }
            _ => panic!("No keymap loaded"),
        }
    }

    pub fn distinct_reports(&self) -> &keymap::DistinctReports {
        match self {
            LoadedKeymap::Keymap {
                distinct_reports, ..
            } => distinct_reports,
            _ => panic!("No keymap loaded"),
        }
    }

    pub fn tick_until_no_scheduled_events(&mut self) {
        match self {
            LoadedKeymap::Keymap {
                keymap,
                distinct_reports,
                ..
            } => {
                while keymap.has_scheduled_events() {
                    keymap.tick();
                    distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
                }
            }
            _ => panic!("No keymap loaded"),
        }
    }
}
#[derive(Debug, World)]
pub struct KeymapWorld {
    keymap_ncl: String,
    keymap: LoadedKeymap,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        KeymapWorld {
            keymap_ncl: String::new(),
            keymap: LoadedKeymap::NoKeymap,
        }
    }
}

#[derive(Deserialize)]
struct DocstringKeymap {
    config: key::composite::Config,
    keys: Vec<Key>,
}

fn load_keymap(keymap_ncl: &str) -> Keymap {
    match nickel_json_serialization_for_keymap(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        keymap_ncl,
    ) {
        Ok(json) => {
            let keymap_result: serde_json::Result<DocstringKeymap> = serde_json::from_str(&json);
            match keymap_result {
                Ok(keymap) => {
                    let dyn_keys = keymap.keys.into_iter().collect();
                    let context = key::composite::Context::from_config(keymap.config);
                    keymap::Keymap::new(dyn_keys, context)
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

#[given("a keymap.ncl:")]
fn setup_nickel_keymap(world: &mut KeymapWorld, step: &Step) {
    let keymap_ncl = step.docstring().unwrap();
    world.keymap_ncl = keymap_ncl.into();
    world.keymap = LoadedKeymap::keymap(load_keymap(keymap_ncl));
}

fn inputs_from_ncl(keymap_ncl: &str, inputs_ncl: &str) -> Vec<input::Event> {
    match nickel_json_serialization_for_inputs(
        format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
        keymap_ncl,
        inputs_ncl,
    ) {
        Ok(json) => {
            let inputs_result: serde_json::Result<Vec<input::Event>> = serde_json::from_str(&json);
            match inputs_result {
                Ok(inputs) => inputs,
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
    let inputs_ncl = step.docstring().unwrap();
    let inputs = inputs_from_ncl(world.keymap_ncl.as_str(), inputs_ncl);

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

#[then("the output should be equivalent to output from")]
fn check_report_equivalences(world: &mut KeymapWorld, step: &Step) {
    let mut test_keymap = load_keymap(TEST_KEYMAP_NCL);
    let mut expected_reports = keymap::DistinctReports::new();

    let inputs_ncl = step.docstring().unwrap();
    let inputs = inputs_from_ncl(TEST_KEYMAP_NCL, inputs_ncl);

    for input in inputs {
        test_keymap.handle_input(input);
        test_keymap.tick();
        expected_reports.update(test_keymap.report_output().as_hid_boot_keyboard_report());
    }

    world.keymap.tick_until_no_scheduled_events();

    let actual_reports = world.keymap.distinct_reports();
    assert_eq!(&expected_reports, actual_reports);
}

fn main() {
    futures::executor::block_on(
        KeymapWorld::cucumber().filter_run("features/keymap/", |_, _, scenario| {
            !scenario.tags.iter().any(|t| t == "ignore")
        }),
    );
}
