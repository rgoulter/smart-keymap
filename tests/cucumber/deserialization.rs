use cucumber::gherkin::Step;
use cucumber::{then, when, World};

use smart_keymap::key;

mod common;

use common::Deserializer;

#[derive(Debug, Default, cucumber::Parameter)]
#[param(name = "key_type", regex = "(?:composite|simple|tap_hold)::Key")]
enum KeyType {
    Composite,
    #[default]
    Simple,
    TapHold,
}

impl std::str::FromStr for KeyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "composite::Key" => Self::Composite,
            "simple::Key" => Self::Simple,
            "tap_hold::Key" => Self::TapHold,
            invalid => return Err(format!("Invalid `KeyType`: {invalid}")),
        })
    }
}

#[derive(Debug, World)]
pub struct KeymapWorld {
    input_key_type: KeyType,
    input_deserializer: Deserializer,
    input_string: String,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        KeymapWorld {
            input_deserializer: Deserializer::JSON,
            input_key_type: KeyType::Simple,
            input_string: String::new(),
        }
    }
}

#[when(expr = "a {key_type} is deserialized from the {deserializer} string")]
fn deserialize_string(
    world: &mut KeymapWorld,
    step: &Step,
    key_type: KeyType,
    deserializer: Deserializer,
) {
    world.input_key_type = key_type;
    world.input_deserializer = deserializer;
    world.input_string = step.docstring.clone().unwrap();
}

#[then(expr = "the result is same value as deserializing the {deserializer} string")]
fn check_value(world: &mut KeymapWorld, step: &Step, deserializer: Deserializer) {
    match world.input_key_type {
        KeyType::Composite => {
            let deserialized_lhs: key::composite::Key = world
                .input_deserializer
                .from_str(&world.input_string)
                .unwrap();
            let deserialized_rhs: key::composite::Key = deserializer
                .from_str(step.docstring.as_ref().unwrap())
                .unwrap();
            assert_eq!(deserialized_lhs, deserialized_rhs);
        }
        KeyType::Simple => {
            let deserialized_lhs: key::simple::Key = world
                .input_deserializer
                .from_str(&world.input_string)
                .unwrap();
            let deserialized_rhs: key::simple::Key = deserializer
                .from_str(step.docstring.as_ref().unwrap())
                .unwrap();
            assert_eq!(deserialized_lhs, deserialized_rhs);
        }
        KeyType::TapHold => {
            let deserialized_lhs: key::tap_hold::Key = world
                .input_deserializer
                .from_str(&world.input_string)
                .unwrap();
            let deserialized_rhs: key::tap_hold::Key = deserializer
                .from_str(step.docstring.as_ref().unwrap())
                .unwrap();
            assert_eq!(deserialized_lhs, deserialized_rhs);
        }
    }
}

fn main() {
    futures::executor::block_on(KeymapWorld::run("features/deserialization/"));
}
