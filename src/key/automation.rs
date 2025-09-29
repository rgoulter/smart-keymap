use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;

/// Reference for a automation key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// Value describing an automation key execution.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Execution {
    /// The start index into the instructions array.
    pub start: u16,
    /// The number of instructions to execute.
    pub length: u16,
}

/// Definition for a automation key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// The automation instructions to execute when the key is pressed.
    pub automation_instructions: Execution,
}

/// An instruction for a automation key.
#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub enum Instruction {
    /// No operation.
    #[default]
    NoOp,
    /// Press a key.
    Press(key::KeyOutput),
    /// Release a key.
    Release(key::KeyOutput),
    /// Wait for a number of ticks.
    Wait(u16),
}

/// Config for automation keys.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config<const INSTRUCTION_COUNT: usize> {
    /// Concatenation of all the automation key instructions.
    ///
    /// Automation keys' instructions are defined by start+len into this array.
    #[serde(deserialize_with = "deserialize_instructions")]
    pub instructions: [Instruction; INSTRUCTION_COUNT],

    /// Duration (in ticks) of each instruction.
    #[serde(default = "default_instruction_duration")]
    pub instruction_duration: u16,
}

/// Constructs an array of instructions for the given array.
pub const fn instructions<const N: usize, const INSTRUCTION_COUNT: usize>(
    instructions: [Instruction; N],
) -> [Instruction; INSTRUCTION_COUNT] {
    let mut cfg_instructions: [Instruction; INSTRUCTION_COUNT] =
        [Instruction::NoOp; INSTRUCTION_COUNT];

    if N > INSTRUCTION_COUNT {
        panic!("Too many instructions for instructions array");
    }

    let mut i = 0;

    while i < N {
        cfg_instructions[i] = instructions[i];
        i += 1;
    }

    cfg_instructions
}

/// Deserialize instructions.
fn deserialize_instructions<'de, D, const INSTRUCTION_COUNT: usize>(
    deserializer: D,
) -> Result<[Instruction; INSTRUCTION_COUNT], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let instructions_vec: heapless::Vec<Instruction, INSTRUCTION_COUNT> =
        Deserialize::deserialize(deserializer)?;

    let mut instructions_array: [Instruction; INSTRUCTION_COUNT] =
        [Instruction::NoOp; INSTRUCTION_COUNT];
    for (i, instruction) in instructions_vec.iter().enumerate() {
        instructions_array[i] = *instruction;
    }

    Ok(instructions_array)
}

fn default_instruction_duration() -> u16 {
    DEFAULT_INSTRUCTION_DURATION
}

/// The default instruction duration.
pub const DEFAULT_INSTRUCTION_DURATION: u16 = 1;

impl<const INSTRUCTION_COUNT: usize> Config<INSTRUCTION_COUNT> {
    /// Constructs a new default [Config].
    pub const fn new() -> Self {
        Self {
            instructions: [{ Instruction::NoOp }; INSTRUCTION_COUNT],
            instruction_duration: DEFAULT_INSTRUCTION_DURATION,
        }
    }
}

impl<const INSTRUCTION_COUNT: usize> Default for Config<INSTRUCTION_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for automation keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context<const INSTRUCTION_COUNT: usize> {
    config: Config<INSTRUCTION_COUNT>,
}

impl<const INSTRUCTION_COUNT: usize> Context<INSTRUCTION_COUNT> {
    /// Constructs a new [Context] with the given [Config].
    pub const fn from_config(config: Config<INSTRUCTION_COUNT>) -> Self {
        Self { config }
    }
}

impl<const INSTRUCTION_COUNT: usize> key::Context for Context<INSTRUCTION_COUNT> {
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        match event {
            key::Event::Key {
                key_event: Event(execution),
                keymap_index,
            } => key_events_for(self.config, keymap_index, execution),
            _ => key::KeyEvents::no_events(),
        }
    }
}

/// The event type for automation keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event(Execution);

/// Converts the instruction to a scheduled event, if applicable.
pub fn key_events_for<const INSTRUCTION_COUNT: usize>(
    config: Config<INSTRUCTION_COUNT>,
    keymap_index: u16,
    Execution { start, length }: Execution,
) -> key::KeyEvents<Event> {
    let instruction = config.instructions[start as usize];

    let next_key_ev = if length > 1 {
        Some(key::Event::Key {
            keymap_index,
            key_event: Event(Execution {
                start: start + 1,
                length: length - 1,
            }),
        })
    } else {
        None
    };

    match instruction {
        Instruction::NoOp => {
            if let Some(key_ev) = next_key_ev {
                let sch_ev = key::ScheduledEvent::after(config.instruction_duration, key_ev);
                key::KeyEvents::scheduled_event(sch_ev)
            } else {
                key::KeyEvents::no_events()
            }
        }
        Instruction::Press(key_output) => {
            let sch_ev =
                key::ScheduledEvent::immediate(key::Event::Input(input::Event::VirtualKeyPress {
                    key_output,
                }));

            let mut pke = key::KeyEvents::scheduled_event(sch_ev);
            if let Some(key_ev) = next_key_ev {
                let sch_ev = key::ScheduledEvent::after(config.instruction_duration, key_ev);
                pke.add_event(sch_ev);
            }

            pke
        }
        Instruction::Release(key_output) => {
            let sch_ev = key::ScheduledEvent::immediate(key::Event::Input(
                input::Event::VirtualKeyRelease { key_output },
            ));

            let mut pke = key::KeyEvents::scheduled_event(sch_ev);
            if let Some(key_ev) = next_key_ev {
                let sch_ev = key::ScheduledEvent::after(config.instruction_duration, key_ev);
                pke.add_event(sch_ev);
            }

            pke
        }
        Instruction::Wait(ticks) => {
            if let Some(key_ev) = next_key_ev {
                let sch_ev = key::ScheduledEvent::after(ticks, key_ev);
                key::KeyEvents::scheduled_event(sch_ev)
            } else {
                key::KeyEvents::no_events()
            }
        }
    }
}

/// The pending key state type for automation keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for automation keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R: Debug, Keys: Index<usize, Output = Key>, const INSTRUCTION_COUNT: usize> {
    keys: Keys,
    _marker: core::marker::PhantomData<(R, [(); INSTRUCTION_COUNT])>,
}

impl<R: Debug, Keys: Index<usize, Output = Key>, const INSTRUCTION_COUNT: usize>
    System<R, Keys, INSTRUCTION_COUNT>
{
    /// Constructs a new [System].
    pub const fn new(keys: Keys) -> Self {
        Self {
            keys,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<R: Copy + Debug, Keys: Debug + Index<usize, Output = Key>, const INSTRUCTION_COUNT: usize>
    key::System<R> for System<R, Keys, INSTRUCTION_COUNT>
{
    type Ref = Ref;
    type Context = Context<INSTRUCTION_COUNT>;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        Ref(key_index): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let pkr = key::PressedKeyResult::Resolved(KeyState);

        let Key {
            automation_instructions: execution,
        } = self.keys[key_index as usize];
        let pke = key_events_for(context.config, keymap_index, execution);

        (pkr, pke)
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        key::KeyEvents::no_events()
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(4, core::mem::size_of::<Event>());
    }
}
