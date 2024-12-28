use serde::Deserialize;

use crate::input;
use crate::key;

pub type LayerIndex = usize;

/// Modifier layer key affects what layers are active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ModifierKey<const L: LayerIndex> {
    Hold(LayerIndex),
}

impl<const L: LayerIndex> ModifierKey<L> {
    pub fn new_pressed_key(
        &self,
        keymap_index: u16,
    ) -> (PressedModifierKey<L>, key::ScheduledEvent<LayerEvent>) {
        match self {
            ModifierKey::Hold(layer) => {
                let event = LayerEvent::LayerActivated(*layer);
                (
                    PressedModifierKey::new(keymap_index),
                    key::ScheduledEvent::immediate(key::Event::Key(event)),
                )
            }
        }
    }
}

impl<const L: LayerIndex> key::Key for ModifierKey<L> {
    type Context = ();
    type PressedKey = PressedModifierKey<L>;
    type Event = LayerEvent;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, Option<key::ScheduledEvent<Self::Event>>) {
        let (pk, ev) = self.new_pressed_key(keymap_index);
        (pk, Some(ev))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context<const L: LayerIndex, C: key::Context> {
    active_layers: [bool; L],
    inner_context: C,
}

impl<const L: LayerIndex, C: key::Context> Context<L, C> {
    pub fn new(inner_context: C) -> Self {
        Self {
            active_layers: [false; L],
            inner_context,
        }
    }

    pub fn active_layers(&self) -> &[bool; L] {
        &self.active_layers
    }
}

impl<const L: LayerIndex, C: key::Context> key::Context for Context<L, C> {
    type Event = LayerEvent;

    fn handle_event(&mut self, event: Self::Event) {
        match event {
            LayerEvent::LayerActivated(layer) => {
                self.active_layers[layer] = true;
            }
            LayerEvent::LayerDeactivated(layer) => {
                self.active_layers[layer] = false;
            }
        }
    }
}

/// A key whose behavior depends on which layer is active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct LayeredKey<const L: LayerIndex, K: key::Key>
where
    [Option<K>; L]: serde::de::DeserializeOwned,
{
    base: K,
    layered: [Option<K>; L],
}

impl<const L: LayerIndex, K: key::Key> LayeredKey<L, K>
where
    [Option<K>; L]: serde::de::DeserializeOwned,
{
    fn new_pressed_key(
        &self,
        context: &Context<L, K::Context>,
        keymap_index: u16,
    ) -> (K::PressedKey, Option<key::ScheduledEvent<K::Event>>) {
        for index in 1..=L {
            let i = L - index;
            if context.active_layers()[i] {
                if let Some(key) = &self.layered[i] {
                    return key.new_pressed_key(&context.inner_context, keymap_index);
                }
            }
        }

        self.base
            .new_pressed_key(&context.inner_context, keymap_index)
    }
}

impl<const L: LayerIndex, K: key::Key> key::Key<K> for LayeredKey<L, K>
where
    [Option<K>; L]: serde::de::DeserializeOwned,
{
    type Context = Context<L, K::Context>;
    type PressedKey = K::PressedKey;
    type Event = K::Event;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, Option<key::ScheduledEvent<Self::Event>>) {
        self.new_pressed_key(context, keymap_index)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum LayerEvent {
    LayerActivated(LayerIndex),
    LayerDeactivated(LayerIndex),
}

#[derive(Debug, Clone, Copy)]
pub struct PressedModifierKey<const L: LayerIndex> {
    keymap_index: u16,
}

impl<const L: LayerIndex> PressedModifierKey<L> {
    pub fn new(keymap_index: u16) -> Self {
        Self { keymap_index }
    }

    fn handle_event(
        &mut self,
        key_definition: &ModifierKey<L>,
        event: key::Event<LayerEvent>,
    ) -> Option<key::Event<LayerEvent>> {
        match key_definition {
            ModifierKey::Hold(layer) => match event {
                key::Event::Input(input::Event::Release { keymap_index }) => {
                    if keymap_index == self.keymap_index {
                        Some(key::Event::Key(LayerEvent::LayerDeactivated(*layer)))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
}

impl<const L: LayerIndex> key::PressedKey<ModifierKey<L>> for PressedModifierKey<L> {
    type Event = LayerEvent;

    fn handle_event(
        &mut self,
        key_definition: &ModifierKey<L>,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        self.handle_event(key_definition, event)
    }

    fn key_code(&self, _key_definition: &ModifierKey<L>) -> Option<u8> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::{simple, Context as _, Key};

    #[test]
    fn test_pressing_hold_modifier_key_emits_event_activate_layer() {
        let layer = 0;
        let key = ModifierKey::<3>::Hold(layer);

        let keymap_index = 9; // arbitrary
        let (_pressed_key, scheduled_event) = key.new_pressed_key(keymap_index);

        if let key::ScheduledEvent {
            event: key::Event::Key(key_ev),
            ..
        } = scheduled_event
        {
            assert_eq!(key_ev, LayerEvent::LayerActivated(layer));
        } else {
            panic!("Expected Some scheduled event");
        }
    }

    #[test]
    fn test_releasing_hold_modifier_key_emits_event_deactivate_layer() {
        // Assemble: press a Hold layer modifier key
        let layer = 0;
        let key = ModifierKey::<3>::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key, _) = key.new_pressed_key(keymap_index);

        // Act: the modifier key handles "release key" input event
        let actual_events = pressed_key.handle_event(
            &key,
            key::Event::Input(input::Event::Release { keymap_index }),
        );

        // Assert: the pressed key should have emitted a layer deactivation event
        if let Some(key::Event::Key(actual_layer_event)) = actual_events {
            let expected_layer_event = LayerEvent::LayerDeactivated(layer);
            assert_eq!(actual_layer_event, expected_layer_event);
        } else {
            panic!("Expected Some LayerDeactivated event");
        }
    }

    #[test]
    fn test_releasing_different_hold_modifier_key_does_not_emit_event() {
        // Assemble: press a Hold layer modifier key
        let layer = 0;
        let key = ModifierKey::<3>::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key, _) = key.new_pressed_key(keymap_index);

        // Act: the modifier key handles "release key" input event for a different key
        let different_keymap_index = keymap_index + 1;
        let different_key_released_ev = key::Event::Input(input::Event::Release {
            keymap_index: different_keymap_index,
        });
        let actual_events = pressed_key.handle_event(&key, different_key_released_ev);

        // Assert: the pressed key should not emit an event
        if actual_events.is_some() {
            panic!("Expected no event emitted");
        }
    }

    #[test]
    fn test_context_handling_event_adjusts_active_layers() {
        let mut context: Context<3, ()> = Context::new(());

        context.handle_event(LayerEvent::LayerActivated(1));

        let actual_active_layers = context.active_layers();
        assert_eq!(&[false, true, false], actual_active_layers);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_base_key_when_no_layers_active() {
        // Assemble
        const L: usize = 3;
        let context: Context<L, ()> = Context::new(());
        let expected_key = simple::Key(0x04);
        let layered_key = LayeredKey {
            base: expected_key,
            layered: [
                Some(simple::Key(0x05)),
                Some(simple::Key(0x06)),
                Some(simple::Key(0x07)),
            ],
        };

        // Act: without activating a layer, press the layered key
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    // Terminology:
    //   "defined layer" = LayeredKey.layered[] is Some for that layer;
    //   "active layer" = Context.active_layers[] = true for that layer.

    #[test]
    fn test_pressing_layered_key_falls_through_undefined_active_layers() {
        // Assemble: layered key (with no layered definitions)
        const L: usize = 3;
        let mut context: Context<L, ()> = Context::new(());
        let expected_key = simple::Key(0x04);
        let layered_key = LayeredKey {
            base: expected_key,
            layered: [None, None, None],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        const L: usize = 3;
        let mut context: Context<L, ()> = Context::new(());
        let expected_key = simple::Key(0x09);
        let layered_key = LayeredKey {
            base: simple::Key(0x04),
            layered: [
                Some(simple::Key(0x05)),
                Some(simple::Key(0x06)),
                Some(expected_key),
            ],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_pressing_layered_key_with_some_transparency_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        const L: usize = 3;
        let mut context: Context<L, ()> = Context::new(());
        let expected_key = simple::Key(0x09);
        let layered_key = LayeredKey {
            base: simple::Key(0x04),
            layered: [Some(expected_key), Some(simple::Key(0x06)), None],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_deserialize_ron_simple() {
        use key::simple;

        let actual_key: key::simple::Key = ron::from_str("Key(0x04)").unwrap();
        let expected_key: key::simple::Key = simple::Key(0x04);
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_option_simple() {
        use key::simple;

        let actual_key: Option<key::simple::Key> = ron::from_str("Some(Key(0x04))").unwrap();
        let expected_key: Option<key::simple::Key> = Some(simple::Key(0x04));
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_array1_u8() {
        let actual: [u8; 1] = ron::from_str("(5)").unwrap();
        let expected: [u8; 1] = [5];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_ron_array1_option_simple() {
        let actual: [Option<key::simple::Key>; 1] = ron::from_str("(Some(Key(0x04)))").unwrap();
        let expected: [Option<key::simple::Key>; 1] = [Some(simple::Key(0x04))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_option_simple() {
        let actual: Option<key::simple::Key> = serde_json::from_str(r#"4"#).unwrap();
        let expected: Option<key::simple::Key> = Some(simple::Key(0x04));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_vec1_option_simple() {
        let actual: heapless::Vec<Option<key::simple::Key>, 1> =
            serde_json::from_str(r#"[4]"#).unwrap();
        let mut expected: heapless::Vec<Option<key::simple::Key>, 1> = heapless::Vec::new();
        expected.push(Some(simple::Key(0x04))).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_array1_option_simple() {
        let actual: [Option<key::simple::Key>; 1] = serde_json::from_str("[4]").unwrap();
        let expected: [Option<key::simple::Key>; 1] = [Some(simple::Key(0x04))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_ron_layered_key_simple_0layer() {
        let actual_key: LayeredKey<0, key::simple::Key> =
            ron::from_str("(base: (0x04), layered: ())").unwrap();
        let expected_key: LayeredKey<0, key::simple::Key> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [],
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_json_layered_key_simple_0layer() {
        let actual_key: LayeredKey<0, key::simple::Key> =
            serde_json::from_str(r#"{"base": 4, "layered": []}"#).unwrap();
        let expected_key: LayeredKey<0, key::simple::Key> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [],
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_layered_key_simple_1layer_none() {
        let actual_key: LayeredKey<1, key::simple::Key> =
            ron::from_str("LayeredKey(base: Key(0x04), layered: (None))").unwrap();
        let expected_key: LayeredKey<1, key::simple::Key> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [None],
        };
        assert_eq!(actual_key, expected_key);
    }
}
