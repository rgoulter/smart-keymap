(import "smart_keys/automation/keymap-codegen.ncl")
& (import "smart_keys/callback/keymap-codegen.ncl")
& (import "smart_keys/caps_word/keymap-codegen.ncl")
& (import "smart_keys/consumer/keymap-codegen.ncl")
& (import "smart_keys/chorded/keymap-codegen.ncl")
& (import "smart_keys/custom/keymap-codegen.ncl")
& (import "smart_keys/keyboard/keymap-codegen.ncl")
& (import "smart_keys/layered/keymap-codegen.ncl")
& (import "smart_keys/mouse/keymap-codegen.ncl")
& (import "smart_keys/sticky/keymap-codegen.ncl")
& (import "smart_keys/tap_dance/keymap-codegen.ncl")
& (import "smart_keys/tap_hold/keymap-codegen.ncl")
& {
  KeymapJson
    | doc "Contract for the keymap.json value"
    = {
      keys | Array smart_key.Json,
      config | composite.config.Json | default = {},
      ..
    },

  validators = import "validators.ncl",

  lib = {
    array_rust_expr = fun data =>
      "[%{data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]",
  },

  smart_keymap,

  json_keymap
    | doc "The 'JSON' value of the keymap. e.g. imported from keymap.json."
    | KeymapJson,

  key_codegen_values =
    json_keymap.keys
    |> std.array.map smart_key.codegen_values,

  key_data_and_refs =
    key_codegen_values
    |> std.array.fold_left
      (fun { key_data, key_refs = refs } cv =>
        let { key_data = kd, ref = r } = smart_key.data_and_ref key_data cv in
        {
          key_data = kd,
          key_refs = refs @ [r],
        }
      )
      { key_data = {}, key_refs = [] }
    |> (fun { key_data, key_refs = refs } =>
      {
        include key_data,
        key_refs = refs |> std.array.map composite.ref.wrap,
      }
    ),

  # DocstringKeymap in cucumber keymap tests.
  json_deserializable_keymap = {
    config = json_keymap.config,
    key_refs =
      key_data_and_refs.key_refs |> std.array.map (fun { json, .. } => json),
    key_data = key_data_and_refs.key_data |> std.record.map_values (std.array.map (fun { json, .. } => json)),
  },

  keyboard_modifiers = {
    Json = std.contract.from_validator json_validator,

    json_validator = validators.is_number,

    is_json = fun json => 'Ok == json_validator json,

    rust_expr = fun b => "smart_keymap::key::KeyboardModifiers::from_byte(%{std.to_string b})",
  },

  mouse_output = {
    Json = std.contract.from_validator json_validator,

    json_validator =
      validators.record.validator {
        fields_validator = validators.record.has_only_fields ["pressed_buttons", "x", "y", "vertical_scroll", "horizontal_scroll"],
        field_validators = {
          pressed_buttons = validators.is_number,
          x = validators.is_number,
          y = validators.is_number,
          vertical_scroll = validators.is_number,
          horizontal_scroll = validators.is_number,
        },
      },

    is_json = fun json => 'Ok == json_validator json,

    rust_expr = fun json =>
      let { pressed_buttons, x, y, vertical_scroll, horizontal_scroll } =
        json
        & {
          pressed_buttons | default = 0,
          x | default = 0,
          y | default = 0,
          vertical_scroll | default = 0,
          horizontal_scroll | default = 0
        }
      in
      m%"
        smart_keymap::key::MouseOutput {
          pressed_buttons: %{std.to_string json.pressed_buttons},
          x: %{std.to_string json.x},
          y: %{std.to_string json.y},
          vertical_scroll: %{std.to_string json.vertical_scroll},
          horizontal_scroll: %{std.to_string json.horizontal_scroll},
        }
      "%,
  },

  checks.check_key_usage = {
    check_json_validation = {
      actual = key_usage.json_validator { Keyboard = 4 },
      expected = 'Ok,
    },
  },

  key_usage = {
    Json = std.contract.from_validator json_validator,

    json_validator =
      validators.record.validator {
        fields_validator = validators.record.has_any_field_of ["Keyboard", "Consumer", "Custom", "MouseOutput"],
        field_validators = {
          Keyboard = validators.is_number,
          Consumer = validators.is_number,
          Custom = validators.is_number,
          MouseOutput = mouse_output.json_validator,
        },
      },

    is_json = fun json => 'Ok == json_validator json,

    rust_expr = fun json =>
      json
      |> match {
        { Keyboard } => "smart_keymap::key::KeyUsage::Keyboard(%{std.to_string Keyboard})",
        { Consumer } => "smart_keymap::key::KeyUsage::Consumer(%{std.to_string Consumer})",
        { Custom } => "smart_keymap::key::KeyUsage::Custom(%{std.to_string Custom})",
        { MouseOutput } => "smart_keymap::key::KeyUsage::Mouse(%{mouse_output.rust_expr MouseOutput})",
      },
  },

  checks.check_key_output = {
    check_json_validation_of_key_code = {
      actual = key_output.json_validator { key_code = { Keyboard = 4 } },
      expected = 'Ok,
    },
  },

  key_output = {
    Json = std.contract.from_validator json_validator,

    json_validator =
      validators.record.validator {
        fields_validator = validators.record.has_only_fields ["key_code", "modifiers"],
        field_validators = {
          key_code = key_usage.json_validator,
          modifiers = keyboard_modifiers.json_validator,
        },
      },

    is_json = fun json => 'Ok == json_validator json,

    rust_expr = fun json =>
      json
      |> match {
        { key_code } => "smart_keymap::key::KeyOutput::from_usage(%{key_usage.rust_expr key_code})",
        { modifiers } =>
          m%"smart_keymap::key::KeyOutput::from_key_modifiers(
          %{keyboard_modifiers.rust_expr modifiers}
        )"%,
        { key_code, modifiers } =>
          m%"smart_keymap::key::KeyOutput::from_usage_with_modifiers(
              %{key_usage.rust_expr key_code},
              %{keyboard_modifiers.rust_expr modifiers}
            )"%,
      },
  },

  composite = {
    ref = {
      wrap = fun ref @ { module, json, rust_expr, .. } =>
        let variant =
          module
          |> match {
            "smart_keymap::key::automation" => "Automation",
            "smart_keymap::key::callback" => "Callback",
            "smart_keymap::key::caps_word" => "CapsWord",
            "smart_keymap::key::chorded" => "Chorded",
            "smart_keymap::key::consumer" => "Consumer",
            "smart_keymap::key::custom" => "Custom",
            "smart_keymap::key::keyboard" => "Keyboard",
            "smart_keymap::key::layered" => "Layered",
            "smart_keymap::key::mouse" => "Mouse",
            "smart_keymap::key::sticky" => "Sticky",
            "smart_keymap::key::tap_dance" => "TapDance",
            "smart_keymap::key::tap_hold" => "TapHold",
            _ => std.fail_with "bad ref for composite.ref.wrap: %{std.serialize ref 'Json}",
          }
        in
        {
          include variant,
          module = "smart_keymap::key::composite",
          json = { "%{variant}" = ref.json },
          rust_expr = "%{module}::Ref::%{variant}(%{ref.rust_expr})",
        },
    },

    config = {
      Json = {
        automation | optional | smart_keymap.automation.config.Json,
        chorded | optional | smart_keymap.chorded.config.Json,
        sticky | optional | smart_keymap.sticky.config.Json,
        tap_dance | optional | smart_keymap.tap_dance.config.Json,
        tap_hold | optional | smart_keymap.tap_hold.config.Json,
      },

      rust_expr = m%"smart_keymap::key::composite::Config {
            automation: %{smart_keymap.automation.config.rust_expr},
            chorded: %{smart_keymap.chorded.config.rust_expr},
            sticky: %{smart_keymap.sticky.config.rust_expr},
            tap_dance: %{smart_keymap.tap_dance.config.rust_expr},
            tap_hold: %{smart_keymap.tap_hold.config.rust_expr},
            ..smart_keymap::key::composite::Config::new()
        }"%,
    },

    system = {
      rust_expr = m%"
        smart_keymap::key::composite::System::array_based(
          %{smart_keymap.automation.system.rust_expr},
          %{smart_keymap.callback.system.rust_expr},
          %{smart_keymap.chorded.system.rust_expr},
          %{smart_keymap.keyboard.system.rust_expr},
          %{smart_keymap.layered.system.rust_expr},
          %{smart_keymap.sticky.system.rust_expr},
          %{smart_keymap.tap_dance.system.rust_expr},
          %{smart_keymap.tap_hold.system.rust_expr},
        )
      "%,
    },
  },

  smart_key = {
    Json = std.contract.from_validator json_validator,

    codegen_modules = [
      smart_keymap.automation.key,
      smart_keymap.callback.key,
      smart_keymap.caps_word.key,
      smart_keymap.chorded.auxiliary_key,
      smart_keymap.chorded.key,
      smart_keymap.consumer.key,
      smart_keymap.custom.key,
      smart_keymap.keyboard.key,
      smart_keymap.mouse.key,
      smart_keymap.layered.layered_key,
      smart_keymap.layered.modifier_key,
      smart_keymap.sticky.key,
      smart_keymap.tap_dance.key,
      smart_keymap.tap_hold.key,
    ],

    json_validator =
      let json_validators = codegen_modules |> std.array.map (fun m => m.json_validator) in
      validators.any_of json_validators,

    is_json = fun json => 'Ok == json_validator json,

    codegen_values = fun json =>
      codegen_modules
      |> std.array.filter (fun m => m.is_json json)
      |> match {
        [m] => m.codegen_values json,
        _ => std.fail_with "bad json for key: %{json |> std.serialize 'Json}",
      },

    codegen_module_for_key_type = fun key_type =>
      codegen_modules
      |> std.array.filter (fun m => m.key_type == key_type)
      |> match {
        [m] => m,
        _ => std.fail_with "bad key_type for smart_key.codegen_module_for_key_type: %{key_type}",
      },

    codegen_module_for_codegen_value = fun cv @ { key_type, .. } =>
      codegen_module_for_key_type key_type,

    traverse = fun f acc cv =>
      let codegen_module = codegen_module_for_codegen_value cv in
      codegen_module.traverse f acc cv,

    data_and_ref = fun key_data cv =>
      let codegen_module = codegen_module_for_codegen_value cv in
      codegen_module.data_and_ref key_data cv,
  },

  rust_expressions
    | doc "Record of rust expressions generated from keymap.json"
    =
      let keymap_len = std.array.length json_keymap.keys in
      let num_automation_instructions =
        if std.record.has_field "automation" json_keymap.config then
          let automation_cfg = json_keymap.config.automation in
          if std.record.has_field "instructions" automation_cfg then
            std.array.length automation_cfg.instructions
          else
            0
        else
          0
      in
      let max_tap_dance_definitions =
        # For each of the keys,
        #  compute max length of any tap dance definitions.
        let max_td_defs =
          std.array.map
            (fun cv =>
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
            key_codegen_values
        in
        # Compute the max over all
        std.array.reduce_left (fun x y => if x > y then x else y) max_td_defs
      in
      let num_layers =
        let max_layered_lengths =
          std.array.map
            (fun cv =>
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
                0
                cv
            )
            key_codegen_values
        in
        # Compute the max over all
        std.array.reduce_left (fun x y => if x > y then x else y) max_layered_lengths
      in
      let chords = ((json_keymap.config & { chorded | default = {} }).chorded & { chords | default = [] }).chords in
      let chord_count = chords |> std.array.length in
      let max_chord_size = chords |> std.array.map std.array.length |> std.array.fold_left std.number.max 0 in
      let max_overlapping_chord_size =
        chords
        |> std.array.map (fun [i, ..] => std.to_string i)
        |> std.array.fold_left
          (fun r i =>
            std.record.update i (1 + (std.record.get_or i 0 r)) r
          )
          {}
        |> std.record.values
        |> std.array.fold_left std.number.max 0
      in
      let { key_data, key_refs } = key_data_and_refs in
      let data_len_rust_expr = fun field_name =>
        let data = (key_data & { "%{field_name}" | default = [] })."%{field_name}" in
        data |> std.array.length |> std.to_string
      in
      let key_refs_expr = "[%{key_refs |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]" in
      {
        config = composite.config.rust_expr,
        context = "smart_keymap::key::composite::Context::from_config(%{config})",
        key_refs = key_refs_expr,
        keymap = m%"
{
    use smart_keymap::key::composite as key_system;
    use key_system::Ref;
    use key_system::Context;
    use key_system::Event;
    use key_system::KeyState;
    use key_system::PendingKeyState;
    use key_system::System;
    const KEY_COUNT: usize = %{keymap_len};
    const KEY_REFS: [Ref; KEY_COUNT] = %{key_refs};
    const CONTEXT: Context = %{context};

    smart_keymap::keymap::Keymap::new(KEY_REFS, CONTEXT, %{composite.system.rust_expr})
}
"%,
        keymap_module = m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = %{std.to_string num_automation_instructions};

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = %{std.to_string num_layers};

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = %{max_chord_size |> std.to_string};

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = %{chord_count |> std.to_string};

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = %{max_overlapping_chord_size |> std.to_string};

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = %{max_tap_dance_definitions |> std.to_string};

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const AUTOMATION: usize = %{data_len_rust_expr "automation"};
    const CALLBACK: usize = %{data_len_rust_expr "callback"};
    const CHORDED: usize = %{data_len_rust_expr "chorded"};
    const CHORDED_AUXILIARY: usize = %{data_len_rust_expr "chorded_auxiliary"};
    const KEYBOARD: usize = %{data_len_rust_expr "keyboard"};
    const LAYERED: usize = %{data_len_rust_expr "layered"};
    const LAYER_MODIFIERS: usize = %{data_len_rust_expr "layer_modifiers"};
    const STICKY: usize = %{data_len_rust_expr "sticky"};
    const TAP_DANCE: usize = %{data_len_rust_expr "tap_dance"};
    const TAP_HOLD: usize = %{data_len_rust_expr "tap_hold"};

    /// The System type
    pub type System = smart_keymap::key::composite::System<
            smart_keymap::key::composite::KeyArrays<
                AUTOMATION,
                CALLBACK,
                CHORDED,
                CHORDED_AUXILIARY,
                KEYBOARD,
                LAYERED,
                LAYER_MODIFIERS,
                STICKY,
                TAP_DANCE,
                TAP_HOLD,
            >,
        >;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = %{std.to_string keymap_len};

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = %{key_refs};

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = %{config};

    /// Initial [Context] value.
    pub const CONTEXT: Context = %{context};

    /// The key system.
    pub const SYSTEM: System = %{composite.system.rust_expr};

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = smart_keymap::keymap::Keymap<
        [Ref; KEY_COUNT],
        Ref,
        Context,
        Event,
        PendingKeyState,
        KeyState,
        System,
    >;
}
"%,
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = rust_expressions.keymap_module,
}
