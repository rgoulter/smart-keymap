# Family registry for the composite key_system shell.
#
# Mergeable fragment:
#  needs `smart_keymap` (for row exprs);
#  contributes `composite.default_family` and `composite.families`.
#
# Capability / shape fields are Nickel enum tags (or small records),
# with related types and exprs nested under the payload:
#
#  - config:         'NoConfig | 'Config { ty, rust_expr }
#                    (struct field path is family `name` / `field`)
#  - context:        { ty, expr }   (always present)
#  - system:         'System { ty, expr }
#                    | 'SystemWithData { data_lengths, rust_expr, ty.array, ty.vec }
#  - pending:        'NoPendingKeyState | 'PendingKeyState { ty }
#  - key_state:      'KeyState { name, ty }  (`name` = KeyState enum arm; default family `variant`)
#  - state_update:   'NoStateUpdate | 'StateUpdate
#  - key_output:     'NoKeyOutput | 'KeyOutput
#  - context_events: 'NoContextEvents | 'ContextEvents
#  - keymap_context: 'NoKeymapContextUpdate | 'UpdatesKeymapContext
{
  smart_keymap,

  composite =
    let camel_case_from_snake_case = fun s =>
      s
      |> std.string.split "_"
      |> std.array.map (fun part =>
        let chars = std.string.characters part in
        let head = std.array.first chars in
        let tail = std.array.drop_first chars in
        "%{std.string.uppercase head}%{std.string.join "" tail}"
      )
      |> std.string.join ""
    in
    {
      # Shared defaults for a key family.
      # Applied at use sites: `{ name = family_name } & families.<name> & default_family`.
      # Lazy fields see `name` / `variant` / `module` from the merged record (like Nickel `| default`).
      # Required from the left-hand / partial:
      #  system, context (and `name` from the resolver).
      default_family = {
        name,
        # e.g. "tap_hold" → "TapHold"
        variant | default = camel_case_from_snake_case name,
        system,
        context,

        module | default = "smart_keymap::key::%{name}",
        field | default = name,

        # Capabilities (enum tags; partials set the affirmative arm only).
        config | default = 'NoConfig,
        pending | default = 'NoPendingKeyState,
        state_update | default = 'NoStateUpdate,
        key_output | default = 'NoKeyOutput,
        context_events | default = 'NoContextEvents,
        keymap_context | default = 'NoKeymapContextUpdate,

        # KeyState enum arm name + Rust type. (`name` not `variant`: nested
        # record would shadow the outer family `variant` field.)
        key_state
          | default
          =
            'KeyState {
              name = variant,
              ty = "%{module}::KeyState",
            },

        ref_ty | default = "%{module}::Ref",
        event_ty | default = "%{module}::Event",
      },

      # Family registry as a record keyed by snake_case name
      #  (stable alphabetical order via std.record.fields).
      # Partials only — merge `default_family` when resolving.
      # Local `module` lets keep `%{module}` type strings without baking defaults here.
      families = {
        automation = {
          module = "smart_keymap::key::automation",
          config =
            'Config {
              ty = m%"%{module}::Config<
            { crate::init::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
              rust_expr = smart_keymap.automation.config.rust_expr,
            },
          state_update = 'StateUpdate,
          context_events = 'ContextEvents,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "AUTOMATION", data_field = "automation" }],
              rust_expr = smart_keymap.automation.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; crate::init::AUTOMATION],
            { crate::init::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key>,
            { crate::init::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { crate::init::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
            expr = "%{module}::Context::from_config(config.automation)",
          },
        },
        callback = {
          module = "smart_keymap::key::callback",
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "CALLBACK", data_field = "callback" }],
              rust_expr = smart_keymap.callback.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; crate::init::CALLBACK]
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key>
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context",
          },
        },
        caps_word = {
          module = "smart_keymap::key::caps_word",
          context_events = 'ContextEvents,
          system =
            'System {
              ty = "%{module}::System<Ref>",
              expr = "%{module}::System::new()",
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::new()",
          },
        },
        chorded = {
          module = "smart_keymap::key::chorded",
          config =
            'Config {
              ty = m%"%{module}::Config<
            { crate::init::CHORDED_MAX_CHORDS },
            { crate::init::CHORDED_MAX_CHORD_SIZE }
          >"%,
              rust_expr = smart_keymap.chorded.config.rust_expr,
            },
          pending =
            'PendingKeyState {
              ty = m%"%{module}::PendingKeyState<
            { crate::init::CHORDED_MAX_CHORDS },
            { crate::init::CHORDED_MAX_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
            },
          context_events = 'ContextEvents,
          keymap_context = 'UpdatesKeymapContext,
          system =
            'SystemWithData {
              data_lengths = [
                { const_name = "CHORDED", data_field = "chorded" },
                { const_name = "CHORDED_AUXILIARY", data_field = "chorded_auxiliary" },
              ],
              rust_expr = smart_keymap.chorded.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [
              %{module}::Key<
                Ref,
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >;
              crate::init::CHORDED
            ],
            [
              %{module}::AuxiliaryKey<
                Ref,
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >;
              crate::init::CHORDED_AUXILIARY
            ],
            { crate::init::CHORDED_MAX_CHORDS },
            { crate::init::CHORDED_MAX_CHORD_SIZE },
            { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<
              %{module}::Key<
                Ref,
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >
            >,
            Vec<
              %{module}::AuxiliaryKey<
                Ref,
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >
            >,
            { crate::init::CHORDED_MAX_CHORDS },
            { crate::init::CHORDED_MAX_CHORD_SIZE },
            { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { crate::init::CHORDED_MAX_CHORDS },
            { crate::init::CHORDED_MAX_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
            expr = "%{module}::Context::from_config(config.chorded)",
          },
        },
        consumer = {
          module = "smart_keymap::key::consumer",
          state_update = 'StateUpdate,
          key_output = 'KeyOutput,
          system =
            'System {
              ty = "%{module}::System<Ref>",
              expr = "%{module}::System::new()",
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context",
          },
        },
        custom = {
          module = "smart_keymap::key::custom",
          key_output = 'KeyOutput,
          system =
            'System {
              ty = "%{module}::System<Ref>",
              expr = "%{module}::System::new()",
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context",
          },
        },
        keyboard = {
          module = "smart_keymap::key::keyboard",
          state_update = 'StateUpdate,
          key_output = 'KeyOutput,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "KEYBOARD", data_field = "keyboard" }],
              rust_expr = smart_keymap.keyboard.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; crate::init::KEYBOARD]
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key>
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context",
          },
        },
        layered = {
          module = "smart_keymap::key::layered",
          config =
            'Config {
              ty = "%{module}::Config",
              rust_expr = smart_keymap.layered.config.rust_expr,
            },
          state_update = 'StateUpdate,
          key_output = 'KeyOutput,
          context_events = 'ContextEvents,
          key_state =
            'KeyState {
              name = "LayerModifier",
              ty = "%{module}::ModifierKeyState",
            },
          system =
            'SystemWithData {
              data_lengths = [
                { const_name = "LAYERED", data_field = "layered" },
                { const_name = "LAYER_MODIFIERS", data_field = "layer_modifiers" },
              ],
              rust_expr = smart_keymap.layered.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::ModifierKey; crate::init::LAYER_MODIFIERS],
            [
              %{module}::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>;
              crate::init::LAYERED
            ],
            { crate::init::LAYERED_LAYER_COUNT }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::ModifierKey>,
            Vec<%{module}::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>>,
            { crate::init::LAYERED_LAYER_COUNT }
          >"%,
            },
          context = {
            ty = "%{module}::Context<{ crate::init::LAYERED_LAYER_COUNT }>",
            expr = "%{module}::Context::from_config(config.layered)",
          },
          event_ty = "%{module}::LayerEvent",
        },
        mouse = {
          module = "smart_keymap::key::mouse",
          key_output = 'KeyOutput,
          system =
            'System {
              ty = "%{module}::System<Ref>",
              expr = "%{module}::System::new()",
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context",
          },
        },
        sticky = {
          module = "smart_keymap::key::sticky",
          config =
            'Config {
              ty = "%{module}::Config",
              rust_expr = smart_keymap.sticky.config.rust_expr,
            },
          state_update = 'StateUpdate,
          key_output = 'KeyOutput,
          context_events = 'ContextEvents,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "STICKY", data_field = "sticky" }],
              rust_expr = smart_keymap.sticky.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; crate::init::STICKY]
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key>
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::from_config(config.sticky)",
          },
        },
        tap_dance = {
          module = "smart_keymap::key::tap_dance",
          config =
            'Config {
              ty = "%{module}::Config",
              rust_expr = smart_keymap.tap_dance.config.rust_expr,
            },
          pending =
            'PendingKeyState {
              ty = "%{module}::PendingKeyState",
            },
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "TAP_DANCE", data_field = "tap_dance" }],
              rust_expr = smart_keymap.tap_dance.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [
              %{module}::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>;
              crate::init::TAP_DANCE
            ],
            { crate::init::TAP_DANCE_MAX_DEFINITIONS }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>>,
            { crate::init::TAP_DANCE_MAX_DEFINITIONS }
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::from_config(config.tap_dance)",
          },
        },
        tap_hold = {
          module = "smart_keymap::key::tap_hold",
          config =
            'Config {
              ty = "%{module}::Config",
              rust_expr = smart_keymap.tap_hold.config.rust_expr,
            },
          pending =
            'PendingKeyState {
              ty = "%{module}::PendingKeyState",
            },
          keymap_context = 'UpdatesKeymapContext,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "TAP_HOLD", data_field = "tap_hold" }],
              rust_expr = smart_keymap.tap_hold.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key<Ref>; crate::init::TAP_HOLD]
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key<Ref>>
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::from_config(config.tap_hold)",
          },
        },
      },
    },
}
