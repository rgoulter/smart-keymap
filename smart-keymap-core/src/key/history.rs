//! History keys: behaviours that depend on previously resolved key output.
//!
//! - [Key::Repeat](crate::key::history::Key::Repeat) re-emits the last remembered
//!   [crate::key::KeyOutput] as the pressed key's own output while held.
//! - [Key::AltRepeat](crate::key::history::Key::AltRepeat) looks up that last
//!   output in a Nickel-defined table ([Config::alt_repeat](crate::key::history::Config::alt_repeat))
//!   and emits the mapped alternate while held (QMK-style alternate repeat for
//!   single keys).

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::key;
use crate::keymap;

/// Reference for a history key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub Key);

/// History key kinds.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Re-emit the last remembered key output while pressed.
    Repeat,
    /// Emit the configured alternate of the last remembered key output while pressed.
    ///
    /// If the last output is unmapped (or history is empty), contributes no output.
    AltRepeat,
}

impl Key {
    /// Constructs a [Key::Repeat].
    pub const fn new_repeat() -> Self {
        Key::Repeat
    }

    /// Constructs a [Key::AltRepeat].
    pub const fn new_alt_repeat() -> Self {
        Key::AltRepeat
    }
}

/// One alternate-repeat mapping: when the last remembered output equals [Self::prev],
/// [Key::AltRepeat] emits [Self::emit].
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct AltRepeatRule {
    /// Previous resolved output that triggers this rule.
    pub prev: key::KeyOutput,
    /// Output to emit instead of repeating [Self::prev].
    pub emit: key::KeyOutput,
}

impl AltRepeatRule {
    /// Empty placeholder rule (used to pad fixed-size arrays).
    pub const EMPTY: Self = Self {
        prev: key::KeyOutput::NO_OUTPUT,
        emit: key::KeyOutput::NO_OUTPUT,
    };

    /// Constructs a rule.
    pub const fn new(prev: key::KeyOutput, emit: key::KeyOutput) -> Self {
        Self { prev, emit }
    }
}

/// Config for history keys (alt-repeat lookup table).
#[derive(Deserialize, Clone, Copy, PartialEq)]
pub struct Config<const ALT_REPEAT_RULE_COUNT: usize> {
    /// Sparse map of previous output → alternate output for [Key::AltRepeat].
    #[serde(deserialize_with = "deserialize_alt_repeat")]
    pub alt_repeat: [AltRepeatRule; ALT_REPEAT_RULE_COUNT],
}

struct AltRepeatDebugHelper<'a, const N: usize> {
    rules: &'a [AltRepeatRule; N],
}

impl<'a, const N: usize> core::fmt::Debug for AltRepeatDebugHelper<'a, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let last_non_empty = self
            .rules
            .iter()
            .rposition(|r| *r != AltRepeatRule::EMPTY)
            .map_or(0, |pos| pos + 1);
        if last_non_empty < N {
            f.debug_list()
                .entries(&self.rules[..last_non_empty])
                .finish_non_exhaustive()
        } else {
            f.debug_list().entries(&self.rules[..]).finish()
        }
    }
}

impl<const ALT_REPEAT_RULE_COUNT: usize> core::fmt::Debug for Config<ALT_REPEAT_RULE_COUNT> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Config")
            .field(
                "alt_repeat",
                &AltRepeatDebugHelper {
                    rules: &self.alt_repeat,
                },
            )
            .finish()
    }
}

/// Builds a fixed-size alt-repeat rule array from a shorter const list (codegen helper).
pub const fn alt_repeat_rules<const N: usize, const ALT_REPEAT_RULE_COUNT: usize>(
    rules: [AltRepeatRule; N],
) -> [AltRepeatRule; ALT_REPEAT_RULE_COUNT] {
    let mut out: [AltRepeatRule; ALT_REPEAT_RULE_COUNT] =
        [AltRepeatRule::EMPTY; ALT_REPEAT_RULE_COUNT];

    if N > ALT_REPEAT_RULE_COUNT {
        panic!("Too many alt-repeat rules for alt_repeat array");
    }

    let mut i = 0;
    while i < N {
        out[i] = rules[i];
        i += 1;
    }
    out
}

fn deserialize_alt_repeat<'de, D, const ALT_REPEAT_RULE_COUNT: usize>(
    deserializer: D,
) -> Result<[AltRepeatRule; ALT_REPEAT_RULE_COUNT], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let rules_vec: heapless::Vec<AltRepeatRule, ALT_REPEAT_RULE_COUNT> =
        Deserialize::deserialize(deserializer)?;

    let mut rules_array: [AltRepeatRule; ALT_REPEAT_RULE_COUNT] =
        [AltRepeatRule::EMPTY; ALT_REPEAT_RULE_COUNT];
    for (i, rule) in rules_vec.iter().enumerate() {
        rules_array[i] = *rule;
    }

    Ok(rules_array)
}

impl<const ALT_REPEAT_RULE_COUNT: usize> Config<ALT_REPEAT_RULE_COUNT> {
    /// Constructs a new default [Config] (empty alt-repeat table).
    pub const fn new() -> Self {
        Self {
            alt_repeat: [AltRepeatRule::EMPTY; ALT_REPEAT_RULE_COUNT],
        }
    }

    /// Looks up the alternate for `prev`, if any rule matches exactly.
    pub fn lookup_alt(&self, prev: &key::KeyOutput) -> Option<key::KeyOutput> {
        self.alt_repeat
            .iter()
            .find(|r| r.prev == *prev && **r != AltRepeatRule::EMPTY)
            .map(|r| r.emit)
    }
}

impl<const ALT_REPEAT_RULE_COUNT: usize> Default for Config<ALT_REPEAT_RULE_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

/// Whether a resolved [key::KeyOutput] should become the remembered last output.
///
/// Empty / no-op outputs are ignored so that pressing Repeat with no history
/// (or keys that resolve without output) does not clear a prior memory.
pub fn is_rememberable(key_output: &key::KeyOutput) -> bool {
    *key_output != key::KeyOutput::NO_OUTPUT
}

/// Context for history keys: tracks the last rememberable resolved output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context<const ALT_REPEAT_RULE_COUNT: usize = 0> {
    /// History / alt-repeat configuration.
    pub config: Config<ALT_REPEAT_RULE_COUNT>,
    last: Option<key::KeyOutput>,
}

impl<const ALT_REPEAT_RULE_COUNT: usize> Default for Context<ALT_REPEAT_RULE_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const ALT_REPEAT_RULE_COUNT: usize> Context<ALT_REPEAT_RULE_COUNT> {
    /// Constructs a new [Context] with default config.
    pub const fn new() -> Self {
        Self::from_config(Config::new())
    }

    /// Constructs a context from the given config.
    pub const fn from_config(config: Config<ALT_REPEAT_RULE_COUNT>) -> Self {
        Context { config, last: None }
    }

    /// Clear remembered history (keeps config).
    pub fn reset(&mut self) {
        *self = Self::from_config(self.config);
    }

    /// The last rememberable resolved key output, if any.
    pub fn last(&self) -> Option<key::KeyOutput> {
        self.last
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        if let key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput { key_output, .. }) = event
        {
            if is_rememberable(&key_output) {
                self.last = Some(key_output);
            }
        }
        key::KeyEvents::no_events()
    }
}

impl<const ALT_REPEAT_RULE_COUNT: usize> key::Context for Context<ALT_REPEAT_RULE_COUNT> {
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }

    fn reset(&mut self) {
        Context::reset(self);
    }
}

/// Events for history keys. (None for v1.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// Pending key state type for history keys. (No pending state.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Pressed state: the output being emitted (if any) for the hold duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    output: Option<key::KeyOutput>,
}

impl KeyState {
    /// Constructs a key state with the given output.
    pub const fn new(output: Option<key::KeyOutput>) -> Self {
        Self { output }
    }

    /// The output this pressed history key contributes, if any.
    pub const fn output(&self) -> Option<key::KeyOutput> {
        self.output
    }
}

/// The [key::System] implementation for history keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, const ALT_REPEAT_RULE_COUNT: usize = 0>(PhantomData<R>);

impl<R, const ALT_REPEAT_RULE_COUNT: usize> System<R, ALT_REPEAT_RULE_COUNT> {
    /// Constructs a new [System].
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R, const ALT_REPEAT_RULE_COUNT: usize> Default for System<R, ALT_REPEAT_RULE_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Debug, const ALT_REPEAT_RULE_COUNT: usize> key::System<R>
    for System<R, ALT_REPEAT_RULE_COUNT>
{
    type Ref = Ref;
    type Context = Context<ALT_REPEAT_RULE_COUNT>;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _keymap_index: u16,
        context: &Self::Context,
        Ref(key): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let output = match key {
            Key::Repeat => context.last(),
            Key::AltRepeat => context
                .last()
                .and_then(|last| context.config.lookup_alt(&last)),
        };
        (
            key::PressedKeyResult::Resolved(KeyState::new(output)),
            key::KeyEvents::no_events(),
        )
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

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        key_state.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::System as _;

    #[test]
    fn test_sizeof_ref() {
        // Two unit variants → one-byte discriminant.
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }

    #[test]
    fn context_remembers_resolved_keyboard_output() {
        let mut ctx = Context::<0>::new();
        assert_eq!(None, ctx.last());

        let key_output = key::KeyOutput::from_key_code(0x04);
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 0,
                key_output,
            }),
        );

        assert_eq!(Some(key_output), ctx.last());
    }

    #[test]
    fn context_ignores_empty_output() {
        let mut ctx = Context::<0>::new();
        let remembered = key::KeyOutput::from_key_code(0x04);
        ctx.last = Some(remembered);

        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 0,
                key_output: key::KeyOutput::NO_OUTPUT,
            }),
        );

        assert_eq!(Some(remembered), ctx.last());
    }

    #[test]
    fn repeat_pressed_key_uses_context_last() {
        let system = System::<(), 0>::new();
        let mut ctx = Context::<0>::new();
        let key_output = key::KeyOutput::from_key_code(0x05);
        ctx.last = Some(key_output);

        let (pkr, _) = system.new_pressed_key(0, &ctx, Ref(Key::Repeat));
        let ks = pkr.unwrap_resolved();
        assert_eq!(Some(key_output), system.key_output(&Ref(Key::Repeat), &ks));
    }

    #[test]
    fn alt_repeat_looks_up_config_rule() {
        let left = key::KeyOutput::from_key_code(0x50);
        let right = key::KeyOutput::from_key_code(0x4F);
        let config = Config {
            alt_repeat: [AltRepeatRule::new(left, right)],
        };
        let mut ctx = Context::from_config(config);
        ctx.last = Some(left);

        let system = System::<(), 1>::new();
        let (pkr, _) = system.new_pressed_key(0, &ctx, Ref(Key::AltRepeat));
        let ks = pkr.unwrap_resolved();
        assert_eq!(Some(right), system.key_output(&Ref(Key::AltRepeat), &ks));
    }

    #[test]
    fn alt_repeat_unmapped_is_none() {
        let system = System::<(), 0>::new();
        let mut ctx = Context::<0>::new();
        ctx.last = Some(key::KeyOutput::from_key_code(0x04));

        let (pkr, _) = system.new_pressed_key(0, &ctx, Ref(Key::AltRepeat));
        let ks = pkr.unwrap_resolved();
        assert_eq!(None, system.key_output(&Ref(Key::AltRepeat), &ks));
    }
}
