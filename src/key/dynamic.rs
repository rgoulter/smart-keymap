use core::fmt::Debug;
use core::marker::PhantomData;

use crate::{input, key};

use key::PressedKey as _;

/// A dyn-compatible Key trait.
pub trait Key<Ev, const M: usize = 2>: Debug
where
    Ev: Copy + Debug + Ord,
{
    /// The context type for the key.
    type Context: key::Context<Event = Ev>;

    /// Handles events in two cases:
    /// - an unpressed key will only receive [input::Event::Press]
    ///   for the keymap index for that key.
    ///   (i.e. an unpressed key won't receive [input::Event::Press] for other keys,
    ///    nor other [input::Event] types),
    /// - a pressed key will receive all kinds of [input::Event].
    fn handle_event(
        &mut self,
        context: Self::Context,
        event: key::Event<Ev>,
    ) -> heapless::Vec<key::ScheduledEvent<Ev>, M>;

    /// Output HID keyboard code for the [Key].
    fn key_output(&self) -> Option<key::KeyOutput>;

    /// Resets the [Key] to its initial state.
    fn reset(&mut self);
}

/// Convenience type alias for [Key] which uses [crate::key::composite::Event] and [crate::key::composite::Context].
pub type CompositeKey<const L: key::layered::LayerIndex = 0> =
    dyn Key<key::composite::Event, Context = key::composite::Context<[bool; L]>>;

/// Generic implementation of [Key] for a [key::Key] and some `Ctx`/`Ev`.
#[derive(Debug)]
pub struct DynamicKey<K: key::Key, Ctx, Ev> {
    key: K,
    pressed_key: Option<input::PressedKey<K, K::PressedKeyState>>,

    _context_type: PhantomData<Ctx>,
    _event_type: PhantomData<Ev>,
}

impl<K: key::Key, Ctx, Ev> DynamicKey<K, Ctx, Ev> {
    /// Constructs a [DynamicKey] with the given key.
    pub const fn new(key: K) -> Self {
        Self {
            key,
            pressed_key: None,

            _context_type: PhantomData,
            _event_type: PhantomData,
        }
    }
}

impl<
        K: key::Key,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + Ord,
        const M: usize,
    > Key<Ev, M> for DynamicKey<K, Ctx, Ev>
where
    key::Event<K::Event>: TryFrom<key::Event<Ev>>,
    key::Event<Ev>: From<key::Event<K::Event>>,
    K::Context: From<Ctx>,
{
    type Context = Ctx;

    fn handle_event(
        &mut self,
        context: Self::Context,
        event: key::Event<Ev>,
    ) -> heapless::Vec<key::ScheduledEvent<Ev>, M> {
        let mut scheduled_events: heapless::Vec<key::ScheduledEvent<Ev>, M> = heapless::Vec::new();

        if let Some(ref mut pressed_key) = &mut self.pressed_key {
            if let Ok(event) = event.try_into() {
                scheduled_events.extend(
                    pressed_key
                        .handle_event(event)
                        .into_iter()
                        .map(|ev| ev.into_scheduled_event()),
                );
            }

            if let key::Event::Input(input::Event::Release { keymap_index }) = event {
                if keymap_index == pressed_key.keymap_index {
                    self.pressed_key = None;
                }
            }
        } else if let key::Event::Input(input::Event::Press { keymap_index }) = event {
            let (pressed_key, new_events) = self.key.new_pressed_key(context.into(), keymap_index);
            scheduled_events.extend(
                new_events
                    .into_iter()
                    .map(|sch_ev| sch_ev.into_scheduled_event()),
            );
            self.pressed_key = Some(pressed_key);
        }

        scheduled_events
    }

    fn key_output(&self) -> Option<key::KeyOutput> {
        if let Some(pressed_key) = &self.pressed_key {
            pressed_key.key_output()
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.pressed_key = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::{composite, simple};

    #[test]
    fn test_composite_dynamic_simple_key_has_no_key_code_when_released() {
        // Assemble
        let dyn_key: &mut dyn Key<composite::Event, Context = composite::Context> =
            &mut DynamicKey::new(simple::Key(0x04));
        let context = composite::Context::default();

        // Act
        let keymap_index: u16 = 5; // arbitrary
        let _ = dyn_key.handle_event(
            context,
            key::Event::Input(input::Event::Press { keymap_index }),
        );
        let _ = dyn_key.handle_event(
            context,
            key::Event::Input(input::Event::Release { keymap_index }),
        );

        // Assert
        let actual_key_code = dyn_key.key_output();
        let expected_key_code = None;
        assert_eq!(actual_key_code, expected_key_code);
    }
}
