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
#  - report_hints:   'NoReportHints | 'ReportHints
#  - init_params:    size / const-generic params emitted in `pub mod init`
#  - module_consts:  private consts inside generated `pub mod key_system`
#                    (e.g. chorded pressed-indices derived from init:: size consts)
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
        report_hints | default = 'NoReportHints,

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

        # Size / const-generic params for `pub mod init` (always emitted).
        # Each: `{ const_name, doc, value = fun ctx => usize }` where
        # `ctx = { json_keymap, key_codegen_values, smart_key }`.
        init_params | default = [],

        # Private consts inside the generated `key_system` module (profile-trimmed).
        # Each: `{ const_name, rust_expr }` — size consts live on parent (`super::…`).
        module_consts | default = [],
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
            { super::AUTOMATION_INSTRUCTION_COUNT }
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
            [%{module}::Key; super::AUTOMATION],
            { super::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key>,
            { super::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { super::AUTOMATION_INSTRUCTION_COUNT }
          >"%,
            expr = "%{module}::Context::from_config(config.automation)",
          },
          init_params = [
            {
              const_name = "AUTOMATION_INSTRUCTION_COUNT",
              doc = "Number of instructions used by the [crate::key::automation] implementation.",
              value = fun { json_keymap, .. } =>
                if std.record.has_field "automation" json_keymap.config then
                  let automation_cfg = json_keymap.config.automation in
                  if std.record.has_field "instructions" automation_cfg then
                    std.array.length automation_cfg.instructions
                  else
                    0
                else
                  0,
            },
          ],
        },
        callback = {
          module = "smart_keymap::key::callback",
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "CALLBACK", data_field = "callback" }],
              rust_expr = smart_keymap.callback.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; super::CALLBACK]
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
            { super::CHORDED_MAX_CHORDS },
            { super::CHORDED_MAX_CHORD_SIZE }
          >"%,
              rust_expr = smart_keymap.chorded.config.rust_expr,
            },
          pending =
            'PendingKeyState {
              ty = m%"%{module}::PendingKeyState<
            { super::CHORDED_MAX_CHORDS },
            { super::CHORDED_MAX_CHORD_SIZE },
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
                { super::CHORDED_MAX_CHORDS },
                { super::CHORDED_MAX_CHORD_SIZE },
                { super::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >;
              super::CHORDED
            ],
            [
              %{module}::AuxiliaryKey<
                Ref,
                { super::CHORDED_MAX_CHORDS },
                { super::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >;
              super::CHORDED_AUXILIARY
            ],
            { super::CHORDED_MAX_CHORDS },
            { super::CHORDED_MAX_CHORD_SIZE },
            { super::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<
              %{module}::Key<
                Ref,
                { super::CHORDED_MAX_CHORDS },
                { super::CHORDED_MAX_CHORD_SIZE },
                { super::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >
            >,
            Vec<
              %{module}::AuxiliaryKey<
                Ref,
                { super::CHORDED_MAX_CHORDS },
                { super::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES
              >
            >,
            { super::CHORDED_MAX_CHORDS },
            { super::CHORDED_MAX_CHORD_SIZE },
            { super::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { super::CHORDED_MAX_CHORDS },
            { super::CHORDED_MAX_CHORD_SIZE },
            CHORDED_MAX_PRESSED_INDICES
          >"%,
            expr = "%{module}::Context::from_config(config.chorded)",
          },
          init_params =
            let chords_from = fun { json_keymap, .. } =>
              ((json_keymap.config & { chorded | default = {} }).chorded & { chords | default = [] }).chords
            in
            [
              {
                const_name = "CHORDED_MAX_CHORD_SIZE",
                doc = "The maximum number of keys in a chord.",
                value = fun ctx =>
                  chords_from ctx
                  |> std.array.map std.array.length
                  |> std.array.fold_left std.number.max 0,
              },
              {
                const_name = "CHORDED_MAX_CHORDS",
                doc = "The maximum number of chords.",
                value = fun ctx => chords_from ctx |> std.array.length,
              },
              {
                const_name = "CHORDED_MAX_OVERLAPPING_CHORD_SIZE",
                doc = "The maximum number of overlapping chords for a chorded key.",
                value = fun ctx =>
                  chords_from ctx
                  |> std.array.map (fun [i, ..] => std.to_string i)
                  |> std.array.fold_left
                    (fun r i =>
                      std.record.update i (1 + (std.record.get_or i 0 r)) r
                    )
                    {}
                  |> std.record.values
                  |> std.array.fold_left std.number.max 0,
              },
            ],
          # Derived inside key_system (not init); used by chorded type params.
          module_consts = [
            {
              const_name = "CHORDED_MAX_PRESSED_INDICES",
              rust_expr = "super::CHORDED_MAX_CHORD_SIZE * 2",
            },
          ],
        },
        consumer = {
          module = "smart_keymap::key::consumer",
          state_update = 'StateUpdate,
          key_output = 'KeyOutput,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "CONSUMER", data_field = "consumer" }],
              rust_expr = smart_keymap.consumer.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; super::CONSUMER]
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
        history = {
          module = "smart_keymap::key::history",
          key_output = 'KeyOutput,
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
        mod_conditioned = {
          module = "smart_keymap::key::mod_conditioned",
          context_events = 'ContextEvents,
          keymap_context = 'UpdatesKeymapContext,
          report_hints = 'ReportHints,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "MOD_CONDITIONED", data_field = "mod_conditioned" }],
              rust_expr = smart_keymap.mod_conditioned.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key<Ref>; super::MOD_CONDITIONED]
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key<Ref>>
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::new()",
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
            [%{module}::Key; super::KEYBOARD]
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
              ty = m%"%{module}::Config<
            { super::CONDITIONAL_LAYER_COUNT }
          >"%,
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
            [%{module}::ModifierKey; super::LAYER_MODIFIERS],
            [
              %{module}::LayeredKey<Ref, { super::LAYERED_LAYER_COUNT }>;
              super::LAYERED
            ],
            { super::LAYERED_LAYER_COUNT },
            { super::CONDITIONAL_LAYER_COUNT }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::ModifierKey>,
            Vec<%{module}::LayeredKey<Ref, { super::LAYERED_LAYER_COUNT }>>,
            { super::LAYERED_LAYER_COUNT },
            { super::CONDITIONAL_LAYER_COUNT }
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { super::LAYERED_LAYER_COUNT },
            { super::CONDITIONAL_LAYER_COUNT }
          >"%,
            expr = "%{module}::Context::from_config(config.layered)",
          },
          event_ty = "%{module}::LayerEvent",
          init_params =
            let conditional_layers_from = fun { json_keymap, .. } =>
              (
                (json_keymap.config & { layered | default = {} }).layered
                & { conditional_layers | default = [] }
              ).conditional_layers
            in
            [
              {
                const_name = "LAYERED_LAYER_COUNT",
                doc = "Number of layers supported by the [smart_keymap::key::layered] implementation.",
                value = fun { key_codegen_values, smart_key, .. } =>
                  # Longest layered array across every key tree (not only the first layered key).
                  key_codegen_values
                  |> std.array.fold_left
                    (fun acc cv =>
                      smart_key.traverse
                        (fun acc cv =>
                          cv
                          |> match {
                            { nested = { layered, .. }, .. } =>
                              let len = std.array.length layered in
                              if len > acc then len else acc,
                            _ => acc,
                          }
                        )
                        acc
                        cv
                    )
                    0,
              },
              {
                const_name = "CONDITIONAL_LAYER_COUNT",
                doc = "Number of conditional layer rules for the [smart_keymap::key::layered] implementation.",
                value = fun ctx => conditional_layers_from ctx |> std.array.length,
              },
            ],
        },
        mouse = {
          module = "smart_keymap::key::mouse",
          key_output = 'KeyOutput,
          system =
            'SystemWithData {
              data_lengths = [{ const_name = "MOUSE", data_field = "mouse" }],
              rust_expr = smart_keymap.mouse.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [%{module}::Key; super::MOUSE]
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
        sequence = {
          module = "smart_keymap::key::sequence",
          config =
            'Config {
              ty = m%"%{module}::Config<
            { super::SEQUENCE_MAX_SEQUENCES },
            { super::SEQUENCE_MAX_SEQUENCE_LEN }
          >"%,
              rust_expr = smart_keymap.sequence.config.rust_expr,
            },
          # Buffer / mode live on Context; no per-key pending session.
          context_events = 'ContextEvents,
          keymap_context = 'UpdatesKeymapContext,
          system =
            'SystemWithData {
              data_lengths = [
                { const_name = "SEQUENCE", data_field = "sequence" },
                { const_name = "SEQUENCE_AUXILIARY", data_field = "sequence_auxiliary" },
              ],
              rust_expr = smart_keymap.sequence.system.rust_expr,
              ty.array = m%"%{module}::System<
            Ref,
            [
              %{module}::Key<
                Ref,
                { super::SEQUENCE_MAX_OVERLAPPING }
              >;
              super::SEQUENCE
            ],
            [
              %{module}::AuxiliaryKey<Ref>;
              super::SEQUENCE_AUXILIARY
            ],
            { super::SEQUENCE_MAX_SEQUENCES },
            { super::SEQUENCE_MAX_SEQUENCE_LEN },
            { super::SEQUENCE_MAX_OVERLAPPING }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<
              %{module}::Key<
                Ref,
                { super::SEQUENCE_MAX_OVERLAPPING }
              >
            >,
            Vec<%{module}::AuxiliaryKey<Ref>>,
            { super::SEQUENCE_MAX_SEQUENCES },
            { super::SEQUENCE_MAX_SEQUENCE_LEN },
            { super::SEQUENCE_MAX_OVERLAPPING }
          >"%,
            },
          context = {
            ty = m%"%{module}::Context<
            { super::SEQUENCE_MAX_SEQUENCES },
            { super::SEQUENCE_MAX_SEQUENCE_LEN }
          >"%,
            expr = "%{module}::Context::from_config(config.sequence)",
          },
          init_params =
            let sequences_from = fun { json_keymap, .. } =>
              ((json_keymap.config & { sequence | default = {} }).sequence & { sequences | default = [] }).sequences
            in
            [
              {
                const_name = "SEQUENCE_MAX_SEQUENCE_LEN",
                doc = "The maximum number of steps in a sequence.",
                value = fun ctx =>
                  sequences_from ctx
                  |> std.array.map std.array.length
                  |> std.array.fold_left std.number.max 0,
              },
              {
                const_name = "SEQUENCE_MAX_SEQUENCES",
                doc = "The maximum number of sequences.",
                value = fun ctx => sequences_from ctx |> std.array.length,
              },
              {
                const_name = "SEQUENCE_MAX_OVERLAPPING",
                doc = "The maximum number of sequences sharing a primary key.",
                value = fun ctx =>
                  sequences_from ctx
                  |> std.array.map (fun [i, ..] => std.to_string i)
                  |> std.array.fold_left
                    (fun r i =>
                      std.record.update i (1 + (std.record.get_or i 0 r)) r
                    )
                    {}
                  |> std.record.values
                  |> std.array.fold_left std.number.max 0,
              },
            ],
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
            [%{module}::Key; super::STICKY]
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
              %{module}::Key<Ref, { super::TAP_DANCE_MAX_DEFINITIONS }>;
              super::TAP_DANCE
            ],
            { super::TAP_DANCE_MAX_DEFINITIONS }
          >"%,
              ty.vec = m%"%{module}::System<
            Ref,
            Vec<%{module}::Key<Ref, { super::TAP_DANCE_MAX_DEFINITIONS }>>,
            { super::TAP_DANCE_MAX_DEFINITIONS }
          >"%,
            },
          context = {
            ty = "%{module}::Context",
            expr = "%{module}::Context::from_config(config.tap_dance)",
          },
          init_params = [
            {
              const_name = "TAP_DANCE_MAX_DEFINITIONS",
              doc = "The tap-dance definitions.",
              value = fun { key_codegen_values, smart_key, .. } =>
                key_codegen_values
                |> std.array.map (fun cv =>
                  smart_key.traverse
                    (fun acc cv =>
                      cv
                      |> match {
                        { nested = { definitions }, .. } =>
                          let len = std.array.length definitions in
                          if len > acc then len else acc,
                        _ => acc,
                      }
                    )
                    0
                    cv
                )
                |> std.array.fold_left (fun x y => if x > y then x else y) 0,
            },
          ],
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
            [%{module}::Key<Ref>; super::TAP_HOLD]
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
