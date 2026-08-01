//! Mod-conditioned keys: dual binding gated on held modifiers,
//!  with optional report-level suppression of those modifiers
//! (ZMK mod-morph / QMK key-override style).
//!
//! At press time, if any of [Key::trigger_mods](crate::key::mod_conditioned::Key::trigger_mods)
//!  is held (from
//!  [KeymapContext::pressed_modifiers](crate::keymap::KeymapContext::pressed_modifiers)),
//!  the key resolves to [Key::morphed](crate::key::mod_conditioned::Key::morphed);
//!  otherwise to [Key::base](crate::key::mod_conditioned::Key::base).
//! On the morph path, trigger mods except
//!  [Key::keep_mods](crate::key::mod_conditioned::Key::keep_mods) are suppressed
//!  from the HID report while the key is held.

use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Maximum simultaneous morph suppress entries.
const MAX_ACTIVE_SUPPRESS: usize = 4;

/// Reference for a mod-conditioned key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// A key that picks a nested binding based on held modifiers.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R> {
    /// Binding used when none of [`Self::trigger_mods`] are held.
    pub base: R,
    /// Binding used when any of [`Self::trigger_mods`] is held.
    pub morphed: R,
    /// Modifiers that activate the morphed binding (any-of).
    #[serde(rename = "mods")]
    pub trigger_mods: key::KeyboardModifiers,
    /// Subset of [`Self::trigger_mods`] to keep in the report on the morph path.
    #[serde(default = "default_keep_mods")]
    pub keep_mods: key::KeyboardModifiers,
}

fn default_keep_mods() -> key::KeyboardModifiers {
    key::KeyboardModifiers::NONE
}

impl<R> Key<R> {
    /// Constructs a mod-conditioned key.
    pub const fn new(
        base: R,
        morphed: R,
        trigger_mods: key::KeyboardModifiers,
        keep_mods: key::KeyboardModifiers,
    ) -> Self {
        Self {
            base,
            morphed,
            trigger_mods,
            keep_mods,
        }
    }

    /// Mods to suppress on the morph path (`trigger \ keep`).
    pub const fn suppress_mods(&self) -> key::KeyboardModifiers {
        self.trigger_mods.difference(&self.keep_mods)
    }
}

/// Events for mod-conditioned keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Morph path taken: suppress these mods for this keymap index while held.
    ActivateSuppress {
        /// Physical keymap index of the conditioned key.
        keymap_index: u16,
        /// Modifier bits to hide from the host report.
        mask: key::KeyboardModifiers,
    },
}

/// Active suppress entry for one held morph.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ActiveSuppress {
    keymap_index: u16,
    mask: key::KeyboardModifiers,
}

/// Context for mod-conditioned keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    pressed_modifiers: key::KeyboardModifiers,
    active: [ActiveSuppress; MAX_ACTIVE_SUPPRESS],
    active_count: u8,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        Context {
            pressed_modifiers: key::KeyboardModifiers::NONE,
            active: [ActiveSuppress {
                keymap_index: 0,
                mask: key::KeyboardModifiers::NONE,
            }; MAX_ACTIVE_SUPPRESS],
            active_count: 0,
        }
    }

    /// Clear runtime state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Currently held keyboard modifiers (engine snapshot).
    pub const fn pressed_modifiers(&self) -> key::KeyboardModifiers {
        self.pressed_modifiers
    }

    /// Union of active suppress masks for the HID report.
    pub fn suppressed_modifiers(&self) -> key::KeyboardModifiers {
        self.active[..self.active_count as usize]
            .iter()
            .fold(key::KeyboardModifiers::NONE, |acc, e| acc.union(&e.mask))
    }

    /// Updates from the engine keymap context snapshot.
    pub fn update_keymap_context(
        &mut self,
        keymap::KeymapContext {
            pressed_modifiers, ..
        }: &keymap::KeymapContext,
    ) {
        self.pressed_modifiers = *pressed_modifiers;
    }

    fn push_suppress(&mut self, keymap_index: u16, mask: key::KeyboardModifiers) {
        if mask == key::KeyboardModifiers::NONE {
            return;
        }
        // Replace existing entry for the same index, if any.
        if let Some(slot) = self.active[..self.active_count as usize]
            .iter_mut()
            .find(|e| e.keymap_index == keymap_index)
        {
            slot.mask = mask;
            return;
        }
        if (self.active_count as usize) < MAX_ACTIVE_SUPPRESS {
            self.active[self.active_count as usize] = ActiveSuppress { keymap_index, mask };
            self.active_count += 1;
        }
    }

    fn clear_suppress(&mut self, keymap_index: u16) {
        let count = self.active_count as usize;
        if let Some(i) = self.active[..count]
            .iter()
            .position(|e| e.keymap_index == keymap_index)
        {
            for j in i..(count - 1) {
                self.active[j] = self.active[j + 1];
            }
            self.active_count -= 1;
        }
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        match event {
            key::Event::Key {
                key_event: Event::ActivateSuppress { keymap_index, mask },
                ..
            } => {
                self.push_suppress(keymap_index, mask);
                key::KeyEvents::no_events()
            }
            key::Event::Input(input::Event::Release { keymap_index }) => {
                self.clear_suppress(keymap_index);
                key::KeyEvents::no_events()
            }
            _ => key::KeyEvents::no_events(),
        }
    }
}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }

    fn reset(&mut self) {
        Context::reset(self);
    }
}

/// Pending key state (none — resolves immediately).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Pressed key state (none — always `NewPressedKey`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for mod-conditioned keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, Keys: Index<usize, Output = Key<R>>> {
    keys: Keys,
}

impl<R, Keys: Index<usize, Output = Key<R>>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    pub const fn new(keys: Keys) -> Self {
        Self { keys }
    }
}

impl<R: Copy + Debug, Keys: Debug + Index<usize, Output = Key<R>>> key::System<R>
    for System<R, Keys>
{
    type Ref = Ref;
    type Context = Context;
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
        let key = &self.keys[key_index as usize];
        let morph = context.pressed_modifiers().has_modifiers(&key.trigger_mods);

        if morph {
            let suppress = key.suppress_mods();
            let pke = if suppress != key::KeyboardModifiers::NONE {
                key::KeyEvents::event(key::Event::key_event(
                    keymap_index,
                    Event::ActivateSuppress {
                        keymap_index,
                        mask: suppress,
                    },
                ))
            } else {
                key::KeyEvents::no_events()
            };
            (
                key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(key.morphed)),
                pke,
            )
        } else {
            (
                key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(key.base)),
                key::KeyEvents::no_events(),
            )
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::System as _;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn morph_when_trigger_mod_held() {
        // Assemble
        let keys = [Key::new(
            0u8, // base
            1u8, // morphed
            key::KeyboardModifiers::LEFT_SHIFT,
            key::KeyboardModifiers::NONE,
        )];
        let system = System::new(keys);
        let mut ctx = Context::new();
        ctx.pressed_modifiers = key::KeyboardModifiers::LEFT_SHIFT;

        // Act
        let (pkr, pke) = system.new_pressed_key(0, &ctx, Ref(0));

        // Assert: morphed binding, and ActivateSuppress for the trigger mod
        assert_eq!(
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(1u8)),
            pkr
        );
        assert_eq!(
            key::KeyEvents::event(key::Event::key_event(
                0,
                Event::ActivateSuppress {
                    keymap_index: 0,
                    mask: key::KeyboardModifiers::LEFT_SHIFT,
                },
            )),
            pke
        );
    }

    #[test]
    fn base_when_no_trigger_mod() {
        // Assemble
        let keys = [Key::new(
            0u8,
            1u8,
            key::KeyboardModifiers::LEFT_SHIFT,
            key::KeyboardModifiers::NONE,
        )];
        let system = System::new(keys);
        let ctx = Context::new();

        // Act
        let (pkr, pke) = system.new_pressed_key(0, &ctx, Ref(0));

        // Assert: base binding, no suppress event
        assert_eq!(
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(0u8)),
            pkr
        );
        assert_eq!(key::KeyEvents::no_events(), pke);
    }

    #[test]
    fn keep_mods_reduces_suppress_mask() {
        let key = Key::new(
            0u8,
            1u8,
            key::KeyboardModifiers::LEFT_SHIFT.union(&key::KeyboardModifiers::RIGHT_SHIFT),
            key::KeyboardModifiers::RIGHT_SHIFT,
        );
        assert_eq!(key::KeyboardModifiers::LEFT_SHIFT, key.suppress_mods());
    }

    #[test]
    fn morph_with_keep_mods_emits_reduced_suppress() {
        // Assemble: trigger LShift|RShift, keep RShift → suppress only LShift
        let keys = [Key::new(
            0u8,
            1u8,
            key::KeyboardModifiers::LEFT_SHIFT.union(&key::KeyboardModifiers::RIGHT_SHIFT),
            key::KeyboardModifiers::RIGHT_SHIFT,
        )];
        let system = System::new(keys);
        let mut ctx = Context::new();
        // Either trigger mod is enough to morph; hold LeftShift
        ctx.pressed_modifiers = key::KeyboardModifiers::LEFT_SHIFT;

        // Act
        let (pkr, pke) = system.new_pressed_key(0, &ctx, Ref(0));

        // Assert: morphed, ActivateSuppress only for the non-kept trigger mod
        assert_eq!(
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(1u8)),
            pkr
        );
        assert_eq!(
            key::KeyEvents::event(key::Event::key_event(
                0,
                Event::ActivateSuppress {
                    keymap_index: 0,
                    mask: key::KeyboardModifiers::LEFT_SHIFT,
                },
            )),
            pke
        );
    }

    #[test]
    fn morph_with_all_keep_mods_emits_no_suppress() {
        // Assemble: keep_mods == trigger_mods → morph with no report suppress
        let keys = [Key::new(
            0u8,
            1u8,
            key::KeyboardModifiers::LEFT_SHIFT,
            key::KeyboardModifiers::LEFT_SHIFT,
        )];
        let system = System::new(keys);
        let mut ctx = Context::new();
        ctx.pressed_modifiers = key::KeyboardModifiers::LEFT_SHIFT;

        // Act
        let (pkr, pke) = system.new_pressed_key(0, &ctx, Ref(0));

        // Assert: still morphed, but no ActivateSuppress event
        assert_eq!(
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(1u8)),
            pkr
        );
        assert_eq!(key::KeyEvents::no_events(), pke);
    }

    #[test]
    fn context_suppress_clears_on_release() {
        // Assemble
        let mut ctx = Context::new();

        // Act: activate suppress for keymap index 3
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::key_event(
                3,
                Event::ActivateSuppress {
                    keymap_index: 3,
                    mask: key::KeyboardModifiers::LEFT_SHIFT,
                },
            ),
        );

        // Assert: suppress active while held
        assert_eq!(
            key::KeyboardModifiers::LEFT_SHIFT,
            ctx.suppressed_modifiers()
        );

        // Act: release the conditioned key
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Input(input::Event::Release { keymap_index: 3 }),
        );

        // Assert: suppress cleared
        assert_eq!(key::KeyboardModifiers::NONE, ctx.suppressed_modifiers());
    }
}
