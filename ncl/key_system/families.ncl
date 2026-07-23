# Family registry for the composite key_system shell.
#
# Function of codegen deps → record of registry/profile fields.
# Included into `composite` by keymap-codegen.ncl so selection/emit share one
# recursive scope (Nickel merge cannot bind sibling fields across files).
#
# Provides default_family, families, Family/Profile contracts, and profile
# builders (profile_from_*, full_profile, keymap_profile).
fun {
  key_codegen_values,
  smart_key,
  smart_keymap,
} =>
  {
    # Shared defaults for a key family.
    # Applied at use sites: `{ name = family_name } & families.<name> & default_family`.
    # Lazy fields see `name` / `variant` / `module` from the merged record (like Nickel `| default`).
    # Required from the left-hand / partial: system_ty.array, context_ty, context_expr
    # (and `name` from the resolver). Const-generic / array lengths use
    # `{ crate::init::… }` in type position (rustc needs braces on paths).
    #
    # Data-carrying families also set `system_ty.vec` (Vec storage); singletons share `system_ty.array`.
    camel_case_from_snake_case = fun s =>
      s
      |> std.string.split "_"
      |> std.array.map (fun part =>
        let chars = std.string.characters part in
        let head = std.array.first chars in
        let tail = std.array.drop_first chars in
        "%{std.string.uppercase head}%{std.string.join "" tail}"
      )
      |> std.string.join "",

    default_family = {
      name,
      # e.g. tap_hold → TapHold
      variant | default = camel_case_from_snake_case name,
      system_ty.array,
      context_ty,
      context_expr,

      module | default = "smart_keymap::key::%{name}",
      field | default = name,
      has_config | default = false,
      config_path | default = name,
      has_pending_dispatch | default = false,
      has_state_update | default = false,
      has_key_output | default = false,
      handles_context_events | default = false,
      updates_keymap_context | default = false,
      key_state_variant | default = variant,
      has_key_data | default = false,
      data_lengths | default = [],
      # Default: same as array system type (singletons / non-data).
      system_ty.vec | default = system_ty.array,

      ref_ty | default = "%{module}::Ref",
      event_ty | default = "%{module}::Event",
      pending_ty | default = "%{module}::PendingKeyState",
      key_state_ty | default = "%{module}::KeyState",
      config_ty | default = "%{module}::Config",
      # Singleton System::new() in struct init body (not key_data system.rust_expr).
      system_expr | default = "%{module}::System::new()",
      # Optional: set on families with has_config / has_key_data (join to smart_keymap.*).
      # Lazy until forced — do not confuse with system_expr above.
    },

    # Family registry as a record keyed by snake_case name (stable alphabetical order
    # via std.record.fields). Partials only — merge `default_family` when resolving.
    # Local `module` lets keep `%{module}` type strings without baking defaults here.
    families = {
      automation =
        let module = "smart_keymap::key::automation" in
        {
          has_config = true,
          has_state_update = true,
          handles_context_events = true,
          has_key_data = true,
          data_lengths = [{ const_name = "AUTOMATION", data_field = "automation" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::Key; { crate::init::AUTOMATION }],
          { crate::init::AUTOMATION_INSTRUCTION_COUNT }
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key>,
          { crate::init::AUTOMATION_INSTRUCTION_COUNT }
        >"%,
          context_ty = m%"%{module}::Context<
          { crate::init::AUTOMATION_INSTRUCTION_COUNT }
        >"%,
          context_expr = "%{module}::Context::from_config(config.automation)",
          config_ty = m%"%{module}::Config<
          { crate::init::AUTOMATION_INSTRUCTION_COUNT }
        >"%,
          config_rust_expr = smart_keymap.automation.config.rust_expr,
          system_rust_expr = smart_keymap.automation.system.rust_expr,
        },
      callback =
        let module = "smart_keymap::key::callback" in
        {
          has_key_data = true,
          data_lengths = [{ const_name = "CALLBACK", data_field = "callback" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::Key; { crate::init::CALLBACK }]
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key>
        >"%,
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context",
          system_rust_expr = smart_keymap.callback.system.rust_expr,
        },
      caps_word =
        let module = "smart_keymap::key::caps_word" in
        {
          handles_context_events = true,
          system_ty.array = "%{module}::System<Ref>",
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context::new()",
        },
      chorded =
        let module = "smart_keymap::key::chorded" in
        {
          has_config = true,
          has_pending_dispatch = true,
          handles_context_events = true,
          updates_keymap_context = true,
          has_key_data = true,
          data_lengths = [
            { const_name = "CHORDED", data_field = "chorded" },
            { const_name = "CHORDED_AUXILIARY", data_field = "chorded_auxiliary" },
          ],
          system_ty.array = m%"%{module}::System<
          Ref,
          [
            %{module}::Key<
              Ref,
              { crate::init::CHORDED_MAX_CHORDS },
              { crate::init::CHORDED_MAX_CHORD_SIZE },
              { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
              CHORDED_MAX_PRESSED_INDICES
            >;
            { crate::init::CHORDED }
          ],
          [
            %{module}::AuxiliaryKey<
              Ref,
              { crate::init::CHORDED_MAX_CHORDS },
              { crate::init::CHORDED_MAX_CHORD_SIZE },
              CHORDED_MAX_PRESSED_INDICES
            >;
            { crate::init::CHORDED_AUXILIARY }
          ],
          { crate::init::CHORDED_MAX_CHORDS },
          { crate::init::CHORDED_MAX_CHORD_SIZE },
          { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
          CHORDED_MAX_PRESSED_INDICES
        >"%,
          system_ty.vec = m%"%{module}::System<
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
          context_ty = m%"%{module}::Context<
          { crate::init::CHORDED_MAX_CHORDS },
          { crate::init::CHORDED_MAX_CHORD_SIZE },
          CHORDED_MAX_PRESSED_INDICES
        >"%,
          context_expr = "%{module}::Context::from_config(config.chorded)",
          pending_ty = m%"%{module}::PendingKeyState<
          { crate::init::CHORDED_MAX_CHORDS },
          { crate::init::CHORDED_MAX_CHORD_SIZE },
          CHORDED_MAX_PRESSED_INDICES
        >"%,
          config_ty = m%"%{module}::Config<
          { crate::init::CHORDED_MAX_CHORDS },
          { crate::init::CHORDED_MAX_CHORD_SIZE }
        >"%,
          config_rust_expr = smart_keymap.chorded.config.rust_expr,
          system_rust_expr = smart_keymap.chorded.system.rust_expr,
        },
      consumer =
        let module = "smart_keymap::key::consumer" in
        {
          has_state_update = true,
          has_key_output = true,
          system_ty.array = "%{module}::System<Ref>",
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context",
        },
      custom =
        let module = "smart_keymap::key::custom" in
        {
          has_key_output = true,
          system_ty.array = "%{module}::System<Ref>",
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context",
        },
      keyboard =
        let module = "smart_keymap::key::keyboard" in
        {
          has_state_update = true,
          has_key_output = true,
          has_key_data = true,
          data_lengths = [{ const_name = "KEYBOARD", data_field = "keyboard" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::Key; { crate::init::KEYBOARD }]
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key>
        >"%,
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context",
          system_rust_expr = smart_keymap.keyboard.system.rust_expr,
        },
      layered =
        let module = "smart_keymap::key::layered" in
        {
          has_config = true,
          has_state_update = true,
          has_key_output = true,
          handles_context_events = true,
          has_key_data = true,
          key_state_variant = "LayerModifier",
          data_lengths = [
            { const_name = "LAYERED", data_field = "layered" },
            { const_name = "LAYER_MODIFIERS", data_field = "layer_modifiers" },
          ],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::ModifierKey; { crate::init::LAYER_MODIFIERS }],
          [
            %{module}::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>;
            { crate::init::LAYERED }
          ],
          { crate::init::LAYERED_LAYER_COUNT }
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::ModifierKey>,
          Vec<%{module}::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>>,
          { crate::init::LAYERED_LAYER_COUNT }
        >"%,
          context_ty = "%{module}::Context<{ crate::init::LAYERED_LAYER_COUNT }>",
          context_expr = "%{module}::Context::from_config(config.layered)",
          event_ty = "%{module}::LayerEvent",
          key_state_ty = "%{module}::ModifierKeyState",
          config_rust_expr = smart_keymap.layered.config.rust_expr,
          system_rust_expr = smart_keymap.layered.system.rust_expr,
        },
      mouse =
        let module = "smart_keymap::key::mouse" in
        {
          has_key_output = true,
          system_ty.array = "%{module}::System<Ref>",
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context",
        },
      sticky =
        let module = "smart_keymap::key::sticky" in
        {
          has_config = true,
          has_state_update = true,
          has_key_output = true,
          handles_context_events = true,
          has_key_data = true,
          data_lengths = [{ const_name = "STICKY", data_field = "sticky" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::Key; { crate::init::STICKY }]
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key>
        >"%,
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context::from_config(config.sticky)",
          config_rust_expr = smart_keymap.sticky.config.rust_expr,
          system_rust_expr = smart_keymap.sticky.system.rust_expr,
        },
      tap_dance =
        let module = "smart_keymap::key::tap_dance" in
        {
          has_config = true,
          has_pending_dispatch = true,
          has_key_data = true,
          data_lengths = [{ const_name = "TAP_DANCE", data_field = "tap_dance" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [
            %{module}::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>;
            { crate::init::TAP_DANCE }
          ],
          { crate::init::TAP_DANCE_MAX_DEFINITIONS }
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>>,
          { crate::init::TAP_DANCE_MAX_DEFINITIONS }
        >"%,
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context::from_config(config.tap_dance)",
          config_rust_expr = smart_keymap.tap_dance.config.rust_expr,
          system_rust_expr = smart_keymap.tap_dance.system.rust_expr,
        },
      tap_hold =
        let module = "smart_keymap::key::tap_hold" in
        {
          has_config = true,
          has_pending_dispatch = true,
          updates_keymap_context = true,
          has_key_data = true,
          data_lengths = [{ const_name = "TAP_HOLD", data_field = "tap_hold" }],
          system_ty.array = m%"%{module}::System<
          Ref,
          [%{module}::Key<Ref>; { crate::init::TAP_HOLD }]
        >"%,
          system_ty.vec = m%"%{module}::System<
          Ref,
          Vec<%{module}::Key<Ref>>
        >"%,
          context_ty = "%{module}::Context",
          context_expr = "%{module}::Context::from_config(config.tap_hold)",
          config_rust_expr = smart_keymap.tap_hold.config.rust_expr,
          system_rust_expr = smart_keymap.tap_hold.system.rust_expr,
        },
    },

    # Resolve a registry partial into a full family record (defaults applied here).
    family = fun family_name =>
      { name = family_name } & families."%{family_name}" & default_family,

    # Resolved families in registry order (alphabetical field order).
    family_list =
      families
      |> std.record.fields
      |> std.array.map family,

    Family = {
      name | String,
      module | String,
      variant | String,
      field | String,
      has_key_data | Bool,
      has_config | Bool,
      config_path | String,
      has_pending_dispatch | Bool,
      has_state_update | Bool,
      has_key_output | Bool,
      handles_context_events | Bool,
      updates_keymap_context | Bool,
      key_state_variant | String,
      ref_ty | String,
      event_ty | String,
      pending_ty | String,
      key_state_ty | String,
      config_ty | String,
      system_ty = {
        array | String,
        vec | String,
      },
      context_ty | String,
      context_expr | String,
      # Singleton System::new() in struct init body.
      system_expr | String,
      # From smart_keymap.*.config / .system (lazy until forced).
      config_rust_expr | String | optional,
      # Required in practice when has_key_data (fail-on-use if missing).
      system_rust_expr | String | optional,
      data_lengths | Array { const_name | String, data_field | String },
    },

    Profile = {
      systems | Array Dyn,
      ref_variants | Array String,
      pending_variants | Array String,
      state_update_variants | Array String,
      key_output_variants | Array String,
      context_event_variants | Array String,
      config_fields | Array String,
      used_modules | Array String,
    },

    used_modules_from = fun codegen_values =>
      codegen_values
      |> std.array.fold_left
        (fun acc cv =>
          smart_key.traverse
            (fun acc cv =>
              if std.record.has_field "module" cv then
                if std.array.elem cv.module acc then
                  acc
                else
                  acc @ [cv.module]
              else
                acc
            )
            acc
            cv
        )
        [],

    profile_from_modules = fun used_modules =>
      let systems =
        family_list
        |> std.array.filter (fun f => std.array.elem f.module used_modules)
      in
      {
        include systems,
        ref_variants = systems |> std.array.map (fun f => f.variant),
        pending_variants =
          systems
          |> std.array.filter (fun f => f.has_pending_dispatch)
          |> std.array.map (fun f => f.variant),
        state_update_variants =
          systems
          |> std.array.filter (fun f => f.has_state_update)
          |> std.array.map (fun f => f.variant),
        key_output_variants =
          systems
          |> std.array.filter (fun f => f.has_key_output)
          |> std.array.map (fun f => f.variant),
        context_event_variants =
          systems
          |> std.array.filter (fun f => f.handles_context_events)
          |> std.array.map (fun f => f.variant),
        config_fields =
          systems
          |> std.array.filter (fun f => f.has_config)
          |> std.array.map (fun f => f.config_path),
        include used_modules,
      } | Profile,

    profile_from_keys = fun keys =>
      keys
      |> std.array.map smart_key.codegen_values
      |> used_modules_from
      |> profile_from_modules,

    # All families in registry order (cucumber / universal shell target).
    full_profile =
      family_list
      |> std.array.map (fun f => f.module)
      |> profile_from_modules,

    # Families used by the current keymap (firmware-trimmed shell).
    keymap_profile =
      key_codegen_values
      |> used_modules_from
      |> profile_from_modules,
  }
