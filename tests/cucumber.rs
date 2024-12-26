use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};

use smart_keymap::key;

#[derive(Debug, World)]
pub struct KeymapWorld {
    input_string: String,
}

impl Default for KeymapWorld {
    fn default() -> Self {
        KeymapWorld {
            input_string: String::new(),
        }
    }
}

#[when("a simple::Key is deserialized from the RON string")]
fn deserialize_string(world: &mut KeymapWorld, step: &Step) {
    world.input_string = step.docstring.clone().unwrap();
}

#[then("the result is same value as deserializng the JSON string")]
fn check_value(world: &mut KeymapWorld, step: &Step) {
    let deserialized_lhs: key::simple::Key = ron::from_str(&world.input_string).unwrap();
    let deserialized_rhs: key::simple::Key =
        serde_json::from_str(step.docstring.as_ref().unwrap()).unwrap();
    assert_eq!(deserialized_lhs, deserialized_rhs);
}

fn main() {
    futures::executor::block_on(KeymapWorld::run("features/"));
}
