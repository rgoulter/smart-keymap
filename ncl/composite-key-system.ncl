# Composite profile + key_system module generation.
#
# Family registry + profile selection + Rust emit for a composite shell.
#
# Profile composition (merge overrides `composite.profile`):
# - Default: `composite.profile` = keymap-stripped (`keymap_profile` from keys).
# - Full:    merge `composite.with_full_profile` (or set
#            `composite.profile = composite.full_profile`) so emit covers every
#            family in `composite.families`.
#
# Trimmed emit → custom keymap `init::key_system` (firmware).
# Full emit → same concrete System shape for now; Keys/KeyVecs backend and
# library `key::composite` cutover are follow-ups.
#
# Naming:
# - `*_ty` — Rust type in type position
# - `context_expr` / `system_expr` — Rust *expressions* (constructors)

{
  validators,

  lib,

  json_keymap,

  key_codegen_values,

  key_data_and_refs,

  smart_key,

  smart_keymap,

  # Shared defaults for a key family. Merge: `{ name = "…", … } & composite.default_family`.
  # Lazy fields see `name` / `variant` / `module` from the merged record (like Nickel `| default`).
  # Required from the left-hand record: name, variant, system_ty, context_ty, context_expr.
  # Const-generic / array lengths use `{ crate::init::… }` in type position (rustc needs braces on paths).
  composite.default_family = {
    name,
    variant,
    system_ty,
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

    ref_ty | default = "%{module}::Ref",
    event_ty | default = "%{module}::Event",
    pending_ty | default = "%{module}::PendingKeyState",
    key_state_ty | default = "%{module}::KeyState",
    config_ty | default = "%{module}::Config",
    system_expr | default = "%{module}::System::new()",
  },

  # Stable match-arm / field order for generated code.
  # Each entry: `{ name = "…", module = "smart_keymap::key::%{name}", … } & default_family`.
  # `module` is on the left so `system_ty` / constructors can use `%{module}` recursively.
  composite.families = [
    {
      name = "automation",
      module = "smart_keymap::key::%{name}",
      variant = "Automation",
      has_config = true,
      has_state_update = true,
      handles_context_events = true,
      has_key_data = true,
      data_lengths = [{ const_name = "AUTOMATION", data_field = "automation" }],
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::Key; { crate::init::AUTOMATION }],
        { crate::init::AUTOMATION_INSTRUCTION_COUNT }
      >"%,
      context_ty = m%"%{module}::Context<
        { crate::init::AUTOMATION_INSTRUCTION_COUNT }
      >"%,
      context_expr = "%{module}::Context::from_config(config.automation)",
      config_ty = m%"%{module}::Config<
        { crate::init::AUTOMATION_INSTRUCTION_COUNT }
      >"%,
    } & composite.default_family,

    {
      name = "callback",
      module = "smart_keymap::key::%{name}",
      variant = "Callback",
      has_key_data = true,
      data_lengths = [{ const_name = "CALLBACK", data_field = "callback" }],
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::Key; { crate::init::CALLBACK }]
      >"%,
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context",
    } & composite.default_family,

    {
      name = "caps_word",
      module = "smart_keymap::key::%{name}",
      variant = "CapsWord",
      handles_context_events = true,
      system_ty = "%{module}::System<Ref>",
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context::new()",
    } & composite.default_family,

    {
      name = "chorded",
      module = "smart_keymap::key::%{name}",
      variant = "Chorded",
      has_config = true,
      has_pending_dispatch = true,
      handles_context_events = true,
      updates_keymap_context = true,
      has_key_data = true,
      data_lengths = [
        { const_name = "CHORDED", data_field = "chorded" },
        { const_name = "CHORDED_AUXILIARY", data_field = "chorded_auxiliary" },
      ],
      system_ty = m%"%{module}::System<
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
    } & composite.default_family,

    {
      name = "consumer",
      module = "smart_keymap::key::%{name}",
      variant = "Consumer",
      has_state_update = true,
      has_key_output = true,
      system_ty = "%{module}::System<Ref>",
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context",
    } & composite.default_family,

    {
      name = "custom",
      module = "smart_keymap::key::%{name}",
      variant = "Custom",
      has_key_output = true,
      system_ty = "%{module}::System<Ref>",
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context",
    } & composite.default_family,

    {
      name = "keyboard",
      module = "smart_keymap::key::%{name}",
      variant = "Keyboard",
      has_state_update = true,
      has_key_output = true,
      has_key_data = true,
      data_lengths = [{ const_name = "KEYBOARD", data_field = "keyboard" }],
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::Key; { crate::init::KEYBOARD }]
      >"%,
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context",
    } & composite.default_family,

    {
      name = "layered",
      module = "smart_keymap::key::%{name}",
      variant = "Layered",
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
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::ModifierKey; { crate::init::LAYER_MODIFIERS }],
        [
          %{module}::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>;
          { crate::init::LAYERED }
        ],
        { crate::init::LAYERED_LAYER_COUNT }
      >"%,
      context_ty = "%{module}::Context<{ crate::init::LAYERED_LAYER_COUNT }>",
      context_expr = "%{module}::Context::from_config(config.layered)",
      event_ty = "%{module}::LayerEvent",
      key_state_ty = "%{module}::ModifierKeyState",
    } & composite.default_family,

    {
      name = "mouse",
      module = "smart_keymap::key::%{name}",
      variant = "Mouse",
      has_key_output = true,
      system_ty = "%{module}::System<Ref>",
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context",
    } & composite.default_family,

    {
      name = "sticky",
      module = "smart_keymap::key::%{name}",
      variant = "Sticky",
      has_config = true,
      has_state_update = true,
      has_key_output = true,
      handles_context_events = true,
      has_key_data = true,
      data_lengths = [{ const_name = "STICKY", data_field = "sticky" }],
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::Key; { crate::init::STICKY }]
      >"%,
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context::from_config(config.sticky)",
    } & composite.default_family,

    {
      name = "tap_dance",
      module = "smart_keymap::key::%{name}",
      variant = "TapDance",
      has_config = true,
      has_pending_dispatch = true,
      has_key_data = true,
      data_lengths = [{ const_name = "TAP_DANCE", data_field = "tap_dance" }],
      system_ty = m%"%{module}::System<
        Ref,
        [
          %{module}::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>;
          { crate::init::TAP_DANCE }
        ],
        { crate::init::TAP_DANCE_MAX_DEFINITIONS }
      >"%,
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context::from_config(config.tap_dance)",
    } & composite.default_family,

    {
      name = "tap_hold",
      module = "smart_keymap::key::%{name}",
      variant = "TapHold",
      has_config = true,
      has_pending_dispatch = true,
      updates_keymap_context = true,
      has_key_data = true,
      data_lengths = [{ const_name = "TAP_HOLD", data_field = "tap_hold" }],
      system_ty = m%"%{module}::System<
        Ref,
        [%{module}::Key<Ref>; { crate::init::TAP_HOLD }]
      >"%,
      context_ty = "%{module}::Context",
      context_expr = "%{module}::Context::from_config(config.tap_hold)",
    } & composite.default_family,
  ],

  composite.Family = {
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
    system_ty | String,
    context_ty | String,
    context_expr | String,
    system_expr | String,
    data_lengths | Array { const_name | String, data_field | String },
  },

  composite.Profile = {
    systems | Array Dyn,
    ref_variants | Array String,
    pending_variants | Array String,
    state_update_variants | Array String,
    key_output_variants | Array String,
    context_event_variants | Array String,
    config_fields | Array String,
    used_modules | Array String,
  },

  composite.used_modules_from = fun codegen_values =>
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

  composite.profile_from_modules = fun used_modules =>
    let systems =
      composite.families
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
    } | composite.Profile,

  composite.profile_from_keys = fun keys =>
    keys
    |> std.array.map smart_key.codegen_values
    |> composite.used_modules_from
    |> composite.profile_from_modules,

  # All families in registry order (cucumber / universal shell target).
  composite.full_profile =
    composite.families
    |> std.array.map (fun f => f.module)
    |> composite.profile_from_modules,

  # Families used by the current keymap (firmware-trimmed shell).
  composite.keymap_profile =
    key_codegen_values
    |> composite.used_modules_from
    |> composite.profile_from_modules,

  # Active profile for `composite.emit`. Default = keymap-stripped.
  # Override via merge: `… & composite.with_full_profile`.
  composite.profile = composite.keymap_profile,

  # Merge this record to select the full registry profile for emit.
  composite.with_full_profile = {
    composite.profile = composite.full_profile,
  },

  composite.config_rust_expr_for = fun name =>
    name
    |> match {
      "automation" => smart_keymap.automation.config.rust_expr,
      "chorded" => smart_keymap.chorded.config.rust_expr,
      "layered" => smart_keymap.layered.config.rust_expr,
      "sticky" => smart_keymap.sticky.config.rust_expr,
      "tap_dance" => smart_keymap.tap_dance.config.rust_expr,
      "tap_hold" => smart_keymap.tap_hold.config.rust_expr,
      _ => std.fail_with "no config.rust_expr for family %{name}",
    },

  composite.system_rust_expr_for = fun name =>
    name
    |> match {
      "automation" => smart_keymap.automation.system.rust_expr,
      "callback" => smart_keymap.callback.system.rust_expr,
      "chorded" => smart_keymap.chorded.system.rust_expr,
      "keyboard" => smart_keymap.keyboard.system.rust_expr,
      "layered" => smart_keymap.layered.system.rust_expr,
      "sticky" => smart_keymap.sticky.system.rust_expr,
      "tap_dance" => smart_keymap.tap_dance.system.rust_expr,
      "tap_hold" => smart_keymap.tap_hold.system.rust_expr,
      _ => std.fail_with "no system.rust_expr for family %{name}",
    },

  # --- Rust emission ---------------------------------------------------------
  # Emit for a profile. Public selection is by composition on `composite.profile`:
  #   default: keymap_profile (stripped)
  #   full:    … & composite.with_full_profile
  # `emit_for_profile` is the shared implementation (also used by checks).

  composite.emit_for_profile = fun profile =>
    let systems = profile.systems in
    let is_full =
      std.array.length systems == std.array.length composite.families
    in
    let join = fun xs => std.string.join "\n" xs in
    let config_systems = systems |> std.array.filter (fun f => f.has_config) in
    let data_array_systems = systems |> std.array.filter (fun f => f.has_key_data) in
    let singleton_systems = systems |> std.array.filter (fun f => f.has_key_data == false) in
    let has_chorded = std.array.any (fun f => f.name == "chorded") systems in

    let enum_variants = fun ty_of =>
      systems
      |> std.array.map (fun f => "/// [%{f.module}] variant.\n%{f.variant}(%{ty_of f}),")
      |> join
    in

    let from_impls = fun aggregate ty_of =>
      systems
      |> std.array.map (fun f =>
        m%"
impl From<%{ty_of f}> for %{aggregate} {
    fn from(v: %{ty_of f}) -> Self { %{aggregate}::%{f.variant}(v) }
}"%
      )
      |> join
    in

    let try_from_impls = fun aggregate ty_of =>
      systems
      |> std.array.map (fun f =>
        m%"
impl TryFrom<%{aggregate}> for %{ty_of f} {
    type Error = smart_keymap::key::EventError;
    fn try_from(v: %{aggregate}) -> Result<Self, Self::Error> {
        match v {
            %{aggregate}::%{f.variant}(v) => Ok(v),
            _ => Err(smart_keymap::key::EventError::UnmappableEvent),
        }
    }
}"%
      )
      |> join
    in

    let config_struct =
      if config_systems == [] then
        m%"
/// Aggregate config (no configurable families in this keymap).
#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {}
impl Default for Config {
    fn default() -> Self { Self::new() }
}
impl Config {
    /// Constructs a new [Config] with defaults.
    pub const fn new() -> Self { Self {} }
}"%
      else
        let fields =
          config_systems
          |> std.array.map (fun f => "/// Config for [%{f.module}].\npub %{f.config_path}: %{f.config_ty},")
          |> join
        in
        let new_fields =
          config_systems
          |> std.array.map (fun f => "%{f.config_path}: %{f.module}::Config::new(),")
          |> join
        in
        m%"
/// Aggregate config for families used by this keymap.
#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
%{fields}
}
impl Default for Config {
    fn default() -> Self { Self::new() }
}
impl Config {
    /// Constructs a new [Config] with defaults.
    pub const fn new() -> Self {
        Self {
%{new_fields}
        }
    }
}"%
    in

    let context_fields =
      (
        ["keymap_context: smart_keymap::keymap::KeymapContext,"]
        @ (systems |> std.array.map (fun f => "%{f.field}: %{f.context_ty},"))
      )
      |> join
    in

    let context_from_config_fields =
      (
        ["keymap_context: smart_keymap::keymap::KeymapContext::new(),"]
        @ (systems |> std.array.map (fun f => "%{f.field}: %{f.context_expr},"))
      )
      |> join
    in

    let context_handle_event =
      systems
      |> std.array.filter (fun f => f.handles_context_events)
      |> std.array.map (fun f =>
        m%"
if let Ok(e) = event.try_into_key_event() {
    pke.extend(self.%{f.field}.handle_event(e).into_events());
}"%
      )
      |> join
    in

    let set_keymap_context_updates =
      systems
      |> std.array.filter (fun f => f.updates_keymap_context)
      |> std.array.map (fun f => "self.%{f.field}.update_keymap_context(&context);")
      |> join
    in

    let system_fields =
      systems
      |> std.array.map (fun f => "%{f.field}: %{f.system_ty},")
      |> join
    in

    let system_new_params =
      data_array_systems
      |> std.array.map (fun f => "%{f.field}: %{f.system_ty},")
      |> join
    in

    let system_new_body =
      (
        (data_array_systems |> std.array.map (fun f => "%{f.field},"))
        @ (singleton_systems |> std.array.map (fun f => "%{f.field}: %{f.system_expr},"))
      )
      |> join
    in

    let new_pressed_key_arms =
      systems
      |> std.array.map (fun f =>
        m%"
Ref::%{f.variant}(key_ref) => {
    let (pkr, pke) = self.%{f.field}.new_pressed_key(keymap_index, &context.%{f.field}, key_ref);
    (pkr.into_result(), pke.into_events())
}"%
      )
      |> join
    in

    let update_pending_arms =
      let pending_systems = systems |> std.array.filter (fun f => f.has_pending_dispatch) in
      if pending_systems == [] then
        "_ => panic!(\"no pending key systems in this key_system\"),"
      else
        (
          pending_systems
          |> std.array.map (fun f =>
            m%"
(Ref::%{f.variant}(key_ref), PendingKeyState::%{f.variant}(pending_state)) => {
    if let Ok(event) = event.try_into_key_event() {
        let (maybe_npk, pke) = self.%{f.field}.update_pending_state(
            pending_state, keymap_index, &context.%{f.field}, key_ref, event,
        );
        (maybe_npk, pke.into_events())
    } else {
        (None, smart_keymap::key::KeyEvents::no_events())
    }
}"%
          )
          |> join
        )
        ++ "\n(_, _) => panic!(\"mismatched key_ref and pending_state variants\"),"
    in

    let update_state_arms =
      let state_systems = systems |> std.array.filter (fun f => f.has_state_update) in
      (
        state_systems
        |> std.array.map (fun f =>
          m%"
(Ref::%{f.variant}(key_ref), KeyState::%{f.key_state_variant}(key_state)) => {
    if let Ok(event) = event.try_into_key_event() {
        self.%{f.field}
            .update_state(key_state, key_ref, &context.%{f.field}, keymap_index, event)
            .into_events()
    } else {
        smart_keymap::key::KeyEvents::no_events()
    }
}"%
        )
        |> join
      )
      ++ "\n(_, _) => smart_keymap::key::KeyEvents::no_events(),"
    in

    let key_output_arms =
      let out_systems = systems |> std.array.filter (fun f => f.has_key_output) in
      (
        out_systems
        |> std.array.map (fun f =>
          "(Ref::%{f.variant}(r), KeyState::%{f.key_state_variant}(ks)) => self.%{f.field}.key_output(r, ks),"
        )
        |> join
      )
      ++ "\n(_, _) => None,"
    in

    let key_state_from_family =
      systems
      |> std.array.map (fun f =>
        m%"
impl From<%{f.key_state_ty}> for KeyState {
    fn from(ks: %{f.key_state_ty}) -> Self { KeyState::%{f.key_state_variant}(ks) }
}"%
      )
      |> join
    in

    let pending_from_family =
      systems
      |> std.array.map (fun f =>
        m%"
impl From<%{f.pending_ty}> for PendingKeyState {
    fn from(pks: %{f.pending_ty}) -> Self { PendingKeyState::%{f.variant}(pks) }
}"%
      )
      |> join
    in

    let pending_mut_try_from =
      systems
      |> std.array.filter (fun f => f.has_pending_dispatch)
      |> std.array.map (fun f =>
        m%"
impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut %{f.pending_ty} {
    type Error = ();
    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::%{f.variant}(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}"%
      )
      |> join
    in

    let config_rust_expr =
      if config_systems == [] then
        "key_system::Config::new()"
      else
        let fields =
          config_systems
          |> std.array.map (fun f =>
            "%{f.config_path}: %{composite.config_rust_expr_for f.name},"
          )
          |> join
        in
        m%"key_system::Config {
%{fields}
}"%,
    in

    let system_rust_expr =
      if data_array_systems == [] then
        "key_system::System::new()"
      else
        let args =
          data_array_systems
          |> std.array.map (fun f => "%{composite.system_rust_expr_for f.name},")
          |> join
        in
        m%"key_system::System::new(
%{args}
)"%,
    in

    let chorded_pressed_indices_const =
      if has_chorded then
        "const CHORDED_MAX_PRESSED_INDICES: usize = crate::init::CHORDED_MAX_CHORD_SIZE * 2;\n"
      else
        ""
    in

    let key_state_variants =
      systems
      |> std.array.map (fun f =>
        "/// [%{f.module}] key state.\n%{f.key_state_variant}(%{f.key_state_ty}),"
      )
      |> join
    in

    let event_param = if context_handle_event == "" then "_event" else "event" in

    let module_doc =
      if is_full then
        "Full composite key system (generated; all registry families)."
      else
        "Per-keymap composite key system (generated; only families used by this keymap)."
    in

    let module_src = m%"
/// %{module_doc}
pub mod key_system {
    use crate as smart_keymap;
    use smart_keymap::key;
    use smart_keymap::keymap;

%{chorded_pressed_indices_const}
    /// Aggregate key reference.
    #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
    pub enum Ref {
%{enum_variants (fun f => f.ref_ty)}
    }

%{config_struct}

    /// Aggregate context.
    #[derive(Debug, Clone, Copy)]
    pub struct Context {
%{context_fields}
    }

    impl Context {
        /// Constructs a [Context] from the given [Config].
        pub const fn from_config(config: Config) -> Self {
            let _ = &config;
            Self {
%{context_from_config_fields}
            }
        }
    }

    impl Default for Context {
        fn default() -> Self {
            Self::from_config(Config::new())
        }
    }

    impl key::Context for Context {
        type Event = Event;

        fn handle_event(
            &mut self,
            %{event_param}: key::Event<Self::Event>,
        ) -> key::KeyEvents<Self::Event> {
            let mut pke = key::KeyEvents::no_events();
%{context_handle_event}
            pke
        }
    }

    impl keymap::SetKeymapContext for Context {
        fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
            self.keymap_context = context;
%{set_keymap_context_updates}
        }
    }

    /// Aggregate event.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Event {
%{enum_variants (fun f => f.event_ty)}
    }

%{from_impls "Event" (fun f => f.event_ty)}
%{try_from_impls "Event" (fun f => f.event_ty)}

    /// Aggregate pending key state.
    #[derive(Debug, Clone, PartialEq)]
    #[allow(clippy::large_enum_variant)]
    pub enum PendingKeyState {
%{enum_variants (fun f => f.pending_ty)}
    }

%{pending_from_family}
%{pending_mut_try_from}

    /// Aggregate key state.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum KeyState {
        /// No-op key state (e.g. auxiliary chorded keys).
        NoOp,
%{key_state_variants}
    }

    impl From<key::NoOpKeyState> for KeyState {
        fn from(_: key::NoOpKeyState) -> Self {
            KeyState::NoOp
        }
    }

%{key_state_from_family}

    /// Aggregate [key::System] for this keymap.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct System {
%{system_fields}
    }

    impl System {
        /// Constructs the system from data-carrying subsystems.
        #[allow(clippy::too_many_arguments)]
        pub const fn new(
%{system_new_params}
        ) -> Self {
            Self {
%{system_new_body}
            }
        }
    }

    impl key::System<Ref> for System {
        type Ref = Ref;
        type Context = Context;
        type Event = Event;
        type PendingKeyState = PendingKeyState;
        type KeyState = KeyState;

        fn new_pressed_key(
            &self,
            keymap_index: u16,
            context: &Self::Context,
            key_ref: Ref,
        ) -> (
            key::PressedKeyResult<Ref, Self::PendingKeyState, Self::KeyState>,
            key::KeyEvents<Self::Event>,
        ) {
            match key_ref {
%{new_pressed_key_arms}
            }
        }

        fn update_pending_state(
            &self,
            pending_state: &mut Self::PendingKeyState,
            keymap_index: u16,
            context: &Self::Context,
            key_ref: Ref,
            event: key::Event<Self::Event>,
        ) -> (Option<key::NewPressedKey<Ref>>, key::KeyEvents<Self::Event>) {
            match (key_ref, pending_state) {
%{update_pending_arms}
            }
        }

        fn update_state(
            &self,
            key_state: &mut Self::KeyState,
            key_ref: &Self::Ref,
            context: &Self::Context,
            keymap_index: u16,
            event: key::Event<Self::Event>,
        ) -> key::KeyEvents<Self::Event> {
            match (key_ref, key_state) {
%{update_state_arms}
            }
        }

        fn key_output(
            &self,
            key_ref: &Self::Ref,
            key_state: &Self::KeyState,
        ) -> Option<key::KeyOutput> {
            match (key_ref, key_state) {
%{key_output_arms}
            }
        }
    }
}
"%
    in

    let size_consts = fun data_len_rust_expr_for_field =>
      systems
      |> std.array.fold_left
        (fun acc f =>
          acc
          @ (
            f.data_lengths
            |> std.array.map (fun { const_name, data_field } =>
              "    const %{const_name}: usize = %{data_len_rust_expr_for_field data_field};"
            )
          )
        )
        []
      |> std.string.join "\n"
    in

    {
      include config_rust_expr,
      include system_rust_expr,
      include module_src,
      include size_consts,
    },

  # Emit for the active `composite.profile` (override via with_full_profile).
  composite.emit = composite.emit_for_profile composite.profile,

  checks.composite_profile = {
    # Inline fixture keys (avoid importing tests/ncl/*.json here — forced when
    # keymap dirs are on the Nickel import path during keymap-ncl-to-json).
    check_1key_simple_keyboard_only =
      let profile = composite.profile_from_keys [{ key_code = 4 }] in
      {
        check_ref_variants = {
          actual = profile.ref_variants,
          expected = ["Keyboard"],
        },
        check_pending_empty = {
          actual = profile.pending_variants,
          expected = [],
        },
        check_config_empty = {
          actual = profile.config_fields,
          expected = [],
        },
      },

    check_1key_tap_hold =
      let profile =
        composite.profile_from_keys [
          {
            hold = { key_code = 224 },
            tap = { key_code = 4 },
          }
        ]
      in
      {
        check_ref_variants = {
          actual = profile.ref_variants,
          expected = ["Keyboard", "TapHold"],
        },
        check_pending = {
          actual = profile.pending_variants,
          expected = ["TapHold"],
        },
        check_config = {
          actual = profile.config_fields,
          expected = ["tap_hold"],
        },
      },

    check_1key_2layer_th_lmod =
      let profile =
        composite.profile_from_keys [
          {
            base = {
              hold = { modifiers = 1 },
              tap = { key_code = 4 },
            },
            layered = [{ Default = 1 }],
          }
        ]
      in
      {
        check_ref_variants = {
          actual = profile.ref_variants,
          expected = ["Keyboard", "Layered", "TapHold"],
        },
        check_pending = {
          actual = profile.pending_variants,
          expected = ["TapHold"],
        },
      },

    # Full registry profile (independent of keymap keys).
    check_full_profile =
      let profile = composite.full_profile in
      {
        check_family_count = {
          actual = std.array.length profile.systems,
          expected = std.array.length composite.families,
        },
        check_ref_variants = {
          actual = profile.ref_variants,
          expected = [
            "Automation",
            "Callback",
            "CapsWord",
            "Chorded",
            "Consumer",
            "Custom",
            "Keyboard",
            "Layered",
            "Mouse",
            "Sticky",
            "TapDance",
            "TapHold",
          ],
        },
        check_pending = {
          actual = profile.pending_variants,
          expected = ["Chorded", "TapDance", "TapHold"],
        },
        check_config = {
          actual = profile.config_fields,
          expected = [
            "automation",
            "chorded",
            "layered",
            "sticky",
            "tap_dance",
            "tap_hold",
          ],
        },
      },
  },

  # Full-profile emit (same concrete System shape as trimmed; Keys/KeyVecs later).
  # Selection form for tools: merge `composite.with_full_profile` then use
  # `composite.emit`; checks call `emit_for_profile full_profile` directly.
  checks.composite_full_emit =
    let module_src = (composite.emit_for_profile composite.full_profile).module_src in
    {
      check_module_doc_full = {
        actual = std.string.is_match "Full composite key system" module_src,
        expected = true,
      },
      check_has_all_ref_variants = {
        actual =
          [
            "Automation",
            "Callback",
            "CapsWord",
            "Chorded",
            "Consumer",
            "Custom",
            "Keyboard",
            "Layered",
            "Mouse",
            "Sticky",
            "TapDance",
            "TapHold",
          ]
          |> std.array.all (fun needle => std.string.is_match needle module_src),
        expected = true,
      },
      check_concrete_system_new = {
        actual = std.string.is_match "pub const fn new" module_src,
        expected = true,
      },
      check_not_keys_trait_yet = {
        actual = std.string.is_match "trait Keys" module_src,
        expected = false,
      },
    },
}
