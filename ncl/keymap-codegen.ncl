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
& (import "composite-key-system.ncl")
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

    # Build `Module::Config { field: expr, ..Module::Config::new() }` from a
    # Nickel record of field name -> Rust value expression strings.
    # Empty record emits `Module::Config::new()`.
    config_rust_expr = fun module field_exprs =>
      if field_exprs == {} then
        "%{module}::Config::new()"
      else
        let fields =
          field_exprs
          |> std.record.to_array
          |> std.array.map (fun { field, value } => "%{field}: %{value},")
          |> std.string.join "\n"
        in
        m%"
        %{module}::Config {
            %{fields}
            ..%{module}::Config::new()
        }
      "%,
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
        fields_validator = validators.record.has_only_fields ["key_code", "key_modifiers"],
        field_validators = {
          key_code = key_usage.json_validator,
          key_modifiers = keyboard_modifiers.json_validator,
        },
      },

    is_json = fun json => 'Ok == json_validator json,

    rust_expr = fun json =>
      json
      |> match {
        { key_code } => "smart_keymap::key::KeyOutput::from_usage(%{key_usage.rust_expr key_code})",
        { key_modifiers } =>
          m%"smart_keymap::key::KeyOutput::from_key_modifiers(
          %{keyboard_modifiers.rust_expr key_modifiers}
        )"%,
        { key_code, key_modifiers } =>
          m%"smart_keymap::key::KeyOutput::from_usage_with_modifiers(
              %{key_usage.rust_expr key_code},
              %{keyboard_modifiers.rust_expr key_modifiers}
            )"%,
      },
  },

  composite = {
    ref = {
      # Wraps a family ref into the per-keymap aggregate Ref.
      # JSON shape stays variant-tagged (same as key::composite) for cucumber;
      # rust_expr targets the generated `key_system::Ref`.
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
          module = "key_system",
          json = { "%{variant}" = ref.json },
          rust_expr = "key_system::Ref::%{variant}(%{ref.rust_expr})",
        },
    },

    config = {
      Json = {
        automation | optional | smart_keymap.automation.config.Json,
        chorded | optional | smart_keymap.chorded.config.Json,
        layered | optional | smart_keymap.layered.config.Json,
        sticky | optional | smart_keymap.sticky.config.Json,
        tap_dance | optional | smart_keymap.tap_dance.config.Json,
        tap_hold | optional | smart_keymap.tap_hold.config.Json,
      },

      # Prefer the trimmed key_system config when emitting keymap.rs.
      rust_expr = composite.emit.config_rust_expr,
    },

    system = {
      rust_expr = composite.emit.system_rust_expr,
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
          0
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
      let key_refs_expr = "[%{key_refs |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]" in
      let config = composite.emit.config_rust_expr in
      let context = "key_system::Context::from_config(%{config})" in
      # Firmware / custom-keymap module: included as `crate::init` with
      # `use crate as smart_keymap` and `crate::init::…` size paths.
      let keymap_module_src = m%"
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

%{composite.emit.size_consts key_data}

%{composite.emit.module_src}

    pub use key_system::Ref;
    pub use key_system::Context;
    pub use key_system::Event;
    pub use key_system::PendingKeyState;
    pub use key_system::KeyState;
    pub use key_system::System;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = %{std.to_string keymap_len};

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = %{key_refs_expr};

    /// The keymap config.
    pub const CONFIG: key_system::Config = %{config};

    /// Initial [Context] value.
    pub const CONTEXT: Context = %{context};

    /// The key system.
    pub const SYSTEM: System = %{composite.emit.system_rust_expr};

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
      in
      # `keymap!` expands at the call site (integration tests, etc.): nested
      # trimmed key_system module + Keymap value — not library full composite.
      # Path rewrites:
      # - `crate as smart_keymap` → external `smart_keymap` crate
      # - `crate::init::` → `super::` (size consts live on the nested module)
      # - drop serde derives (call site may not depend on serde)
      let keymap_expr_src =
        let adapted_module =
          keymap_module_src
          |> std.string.replace "pub mod init" "mod __smart_keymap_keymap"
          |> std.string.replace "use crate as smart_keymap;\n" ""
          |> std.string.replace "use crate as smart_keymap;" ""
          |> std.string.replace "crate::init::" "super::"
          |> std.string.replace "serde::Deserialize, " ""
        in
        m%"
{
%{adapted_module}
    __smart_keymap_keymap::Keymap::new(
        __smart_keymap_keymap::KEY_REFS,
        __smart_keymap_keymap::CONTEXT,
        __smart_keymap_keymap::SYSTEM,
    )
}
"%,
      in
      {
        include config,
        include context,
        key_refs = key_refs_expr,
        keymap = keymap_expr_src,
        keymap_module = keymap_module_src,
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = rust_expressions.keymap_module,
}
