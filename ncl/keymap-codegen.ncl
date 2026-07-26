(import "smart_keys/automation/keymap-codegen.ncl")
& (import "smart_keys/callback/keymap-codegen.ncl")
& (import "smart_keys/caps_word/keymap-codegen.ncl")
& (import "smart_keys/consumer/keymap-codegen.ncl")
& (import "smart_keys/chorded/keymap-codegen.ncl")
& (import "smart_keys/custom/keymap-codegen.ncl")
& (import "smart_keys/history/keymap-codegen.ncl")
& (import "smart_keys/keyboard/keymap-codegen.ncl")
& (import "smart_keys/layered/keymap-codegen.ncl")
& (import "smart_keys/mouse/keymap-codegen.ncl")
& (import "smart_keys/sticky/keymap-codegen.ncl")
& (import "smart_keys/tap_dance/keymap-codegen.ncl")
& (import "smart_keys/tap_hold/keymap-codegen.ncl")
& (import "key_system/families.ncl")
& (import "key_system/keymap-codegen.ncl")
& {
  composite,

  KeymapJson
    | doc "Contract for the keymap.json value"
    =
      let ConfigJson = composite.config.Json in
      {
        keys | Array smart_key.Json,
        config | ConfigJson | default = {},
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
      smart_keymap.history.key,
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
      let { key_data, key_refs } = key_data_and_refs in
      let key_refs_expr = "[%{key_refs |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]" in
      let config_expr = composite.config.rust_expr in
      let context_expr = "key_system::Context::from_config(%{config_expr})" in
      let init_size_ctx = {
        include json_keymap,
        include key_codegen_values,
        include smart_key,
      }
      in
      # Nested shell under `pub mod init`: size/data consts + key_system (uses
      # `super::` for those consts) + keymap instance surface. Engine paths use
      # the `smart_keymap` crate name (facade: `extern crate self as smart_keymap`).
      #
      # Same text for firmware include and `keymap!` (block-local `pub mod init`
      # does not leak). Only serde differs: false for `keymap!` call sites.
      let init_module = fun { serde } =>
        let sys = (composite & { with_serde = serde }).system in
        m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {

%{sys.init_size_consts_rust_stmts init_size_ctx}

%{sys.init_consts_rust_stmts key_data}

%{sys.rust_mod}

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
    pub const CONFIG: key_system::Config = %{config_expr};

    /// Initial [Context] value.
    pub const CONTEXT: Context = %{context_expr};

    /// The key system.
    pub const SYSTEM: System = %{sys.rust_expr};

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
      let keymap_module_src = init_module { serde = true } in
      let keymap_expr = m%"
{
%{init_module { serde = false }}
    init::Keymap::new(
        init::KEY_REFS,
        init::CONTEXT,
        init::SYSTEM,
    )
}
"%,
      in
      {
        config = config_expr,
        context = config_expr,
        key_refs = key_refs_expr,
        keymap = keymap_expr,
        keymap_module = keymap_module_src,
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = rust_expressions.keymap_module,
}
