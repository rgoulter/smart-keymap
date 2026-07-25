# Composite profile selection + key_system module generation.
#
# Family table: families.ncl (merged fragment; static-ish rows + smart_keymap
# expr joins under composite.default_family / composite.families).
#
# Selection is by merge composition:
# - `composite.profile` — which families:
#     `'FullProfile` | `'Profile profile`  (default: `'Profile keymap_profile`)
# - `composite.data` — key-data storage (`'Array` | `'Vec`; default `'Array`).
# - Artefacts for the selected profile + data live on `composite` itself:
#   - `composite.ref.wrap` — family ref → aggregate `key_system::Ref`
#   - `composite.config.Json` / `composite.config.rust_expr`
#   - `composite.system.rust_expr`
#   - `composite.system.init_consts_rust_stmts` (fn of key_data)
#   - `composite.system.init_size_consts_rust_stmts` (fn of ctx; always-emit)
#   - `composite.system.rust_mod` (generated `pub mod key_system { … }`)
#
# Profile:
# - Default: keymap-stripped (`'Profile keymap_profile` from keys).
# - Full:    merge `composite.with_full_profile` (sets `profile = 'FullProfile`).
#
# Data storage (`composite.data`):
# - `'Array` — concrete `System` with `[Key; N]` (firmware trimmed default).
# - `'Vec`   — concrete `System` with `Vec<Key>` (cucumber / runtime serde).
#
# Registry fields come from merging families.ncl; refer via composite.* paths
# (short names are not shared across merge operands).
{
  key_codegen_values,

  smart_key,

  smart_keymap,

  composite = {
    # Resolve a registry partial into a full family record (defaults applied here).
    family = fun family_name =>
      { name = family_name }
      & composite.families."%{family_name}"
      & composite.default_family,

    # Resolved families in registry order (alphabetical field order).
    family_list =
      composite.families
      |> std.record.fields
      |> std.array.map family,

    # Wraps a family ref into the per-keymap aggregate Ref.
    ref.wrap = fun r @ { module, json, rust_expr, .. } =>
      let f =
        family_list
        |> std.array.filter (fun f => f.module == module)
        |> match {
          [f] => f,
          _ =>
            std.fail_with "bad ref for composite.ref.wrap: no family for module %{module} (ref: %{std.serialize r 'Json})",
        }
      in
      {
        variant = f.variant,
        module = "key_system",
        json = { "%{f.variant}" = r.json },
        rust_expr = "key_system::Ref::%{f.variant}(%{r.rust_expr})",
      },

    # Family capability predicates / extractors (enum tags from families.ncl).
    family_has_config = fun f =>
      f.config
      |> match {
        'NoConfig => false,
        'Config _ => true,
      },
    family_config = fun f =>
      f.config
      |> match {
        'Config c => c,
      },
    family_has_pending = fun f =>
      f.pending
      |> match {
        'NoPendingKeyState => false,
        'PendingKeyState _ => true,
      },
    family_pending_ty = fun f =>
      f.pending
      |> match {
        'NoPendingKeyState => "%{f.module}::PendingKeyState",
        'PendingKeyState { ty } => ty,
      },
    family_has_state_update = fun f => f.state_update == 'StateUpdate,
    family_has_key_output = fun f => f.key_output == 'KeyOutput,
    family_has_context_events = fun f => f.context_events == 'ContextEvents,
    family_updates_keymap_context = fun f => f.keymap_context == 'UpdatesKeymapContext,
    family_has_key_data = fun f =>
      f.system
      |> match {
        'System _ => false,
        'SystemWithData _ => true,
      },
    family_key_state = fun f =>
      f.key_state
      |> match {
        'KeyState ks => ks,
      },
    family_system_data = fun f =>
      f.system
      |> match {
        'SystemWithData d => d,
      },
    family_system_expr = fun f =>
      f.system
      |> match {
        'System { expr, .. } => expr,
      },

    Family = {
      name | String,
      module | String,
      variant | String,
      field | String,
      # Shape / capability enums (see families.ncl header).
      config | Dyn,
      pending | Dyn,
      state_update | Dyn,
      key_output | Dyn,
      context_events | Dyn,
      keymap_context | Dyn,
      key_state | Dyn,
      system | Dyn,
      context | { ty | String, expr | String },
      ref_ty | String,
      event_ty | String,
    },

    # Resolved set of families + derived variant lists (content of 'Profile).
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
          |> std.array.filter family_has_pending
          |> std.array.map (fun f => f.variant),
        state_update_variants =
          systems
          |> std.array.filter family_has_state_update
          |> std.array.map (fun f => f.variant),
        key_output_variants =
          systems
          |> std.array.filter family_has_key_output
          |> std.array.map (fun f => f.variant),
        context_event_variants =
          systems
          |> std.array.filter family_has_context_events
          |> std.array.map (fun f => f.variant),
        config_fields =
          systems
          |> std.array.filter family_has_config
          |> std.array.map (fun f => f.field),
        include used_modules,
      } | Profile,

    profile_from_keys = fun keys =>
      keys
      |> std.array.map smart_key.codegen_values
      |> used_modules_from
      |> profile_from_modules,

    # All families in registry order
    # (Used by: cucumber / universal shell target).
    full_profile =
      family_list
      |> std.array.map (fun f => f.module)
      |> profile_from_modules,

    # Families used by the current keymap
    # (Used by: firmware-trimmed shell).
    keymap_profile =
      key_codegen_values
      |> used_modules_from
      |> profile_from_modules,

    Storage =
      std.contract.from_predicate (fun x =>
        x == 'Array || x == 'Vec
      ),

    # Active profile selection: 'FullProfile | 'Profile profile.
    # Default = keymap-stripped.
    profile | default = 'Profile keymap_profile,

    # Key-data storage for composite artefacts. Default = Array (firmware).
    data | Storage | default = 'Array,

    # Merge this record to select the full registry profile.
    with_full_profile = {
      profile = 'FullProfile,
    },

    # Merge this record to select Vec key-data storage.
    with_vec_data = {
      data | Storage = 'Vec,
    },

    # Resolve selection enum to a Profile record.
    resolved_profile =
      profile
      |> match {
        'FullProfile => full_profile,
        'Profile p => p,
      },

    # --- composite fragments (config / system codegen) -------------------------
    # Named fragments over active profile + data.

    system_ty_for = fun storage f =>
      f.system
      |> match {
        'System { ty, .. } => ty,
        'SystemWithData { ty, .. } =>
          storage
          |> match {
            'Array => ty.array,
            'Vec => ty.vec,
          },
      },

    # Module-shaped codegen fragments for the active profile + data.
    fragments = {
      systems = resolved_profile.systems,
      is_full =
        profile
        |> match {
          'FullProfile => true,
          'Profile _ => false,
        },
      is_vec = data == 'Vec,

      join = fun xs => std.string.join "\n" xs,

      config_systems = systems |> std.array.filter family_has_config,
      data_array_systems = systems |> std.array.filter family_has_key_data,
      singleton_systems =
        systems
        |> std.array.filter (fun f => family_has_key_data f == false),
      has_chorded = std.array.any (fun f => f.name == "chorded") systems,

      sty = fun f => system_ty_for data f,

      enum_variants = fun ty_of =>
        systems
        |> std.array.map (fun f => "/// [%{f.module}] variant.\n%{f.variant}(%{ty_of f}),")
        |> join,

      from_impls = fun aggregate ty_of =>
        systems
        |> std.array.map (fun f =>
          m%"
impl From<%{ty_of f}> for %{aggregate} {
    fn from(v: %{ty_of f}) -> Self { %{aggregate}::%{f.variant}(v) }
}"%
        )
        |> join,

      try_from_impls = fun aggregate ty_of =>
        systems
        |> std.array.map (fun f =>
          m%"
#[allow(unreachable_patterns)]
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
        |> join,

      # Vec-backed shells deserialize sparse key_data/config (cucumber).
      config_field_attrs = if is_vec then "\n    #[serde(default)]" else "",

      config_struct =
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
            |> std.array.map (fun f =>
              let c = family_config f in
              "/// Config for [%{f.module}].%{config_field_attrs}\n    pub %{f.field}: %{c.ty},"
            )
            |> join
          in
          let new_fields =
            config_systems
            |> std.array.map (fun f => "%{f.field}: %{f.module}::Config::new(),")
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
}"%,

      context_fields =
        (
          ["keymap_context: smart_keymap::keymap::KeymapContext,"]
          @ (systems |> std.array.map (fun f => "%{f.field}: %{f.context.ty},"))
        )
        |> join,

      context_from_config_fields =
        (
          ["keymap_context: smart_keymap::keymap::KeymapContext::new(),"]
          @ (systems |> std.array.map (fun f => "%{f.field}: %{f.context.expr},"))
        )
        |> join,

      context_handle_event =
        systems
        |> std.array.filter family_has_context_events
        |> std.array.map (fun f =>
          m%"
if let Ok(e) = event.try_into_key_event() {
    pke.extend(self.%{f.field}.handle_event(e).into_events());
}"%
        )
        |> join,

      set_keymap_context_updates =
        systems
        |> std.array.filter family_updates_keymap_context
        |> std.array.map (fun f => "self.%{f.field}.update_keymap_context(&context);")
        |> join,

      system_fields =
        systems
        |> std.array.map (fun f => "%{f.field}: %{sty f},")
        |> join,

      system_new_params =
        data_array_systems
        |> std.array.map (fun f => "%{f.field}: %{sty f},")
        |> join,

      system_new_body =
        (
          (data_array_systems |> std.array.map (fun f => "%{f.field},"))
          @ (
            singleton_systems
            |> std.array.map (fun f =>
              "%{f.field}: %{family_system_expr f},"
            )
          )
        )
        |> join,

      new_pressed_key_arms =
        systems
        |> std.array.map (fun f =>
          m%"
Ref::%{f.variant}(key_ref) => {
    let (pkr, pke) = self.%{f.field}.new_pressed_key(keymap_index, &context.%{f.field}, key_ref);
    (pkr.into_result(), pke.into_events())
}"%
        )
        |> join,

      update_pending_arms =
        let pending_systems =
          systems |> std.array.filter family_has_pending
        in
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
          ++ "\n(_, _) => panic!(\"mismatched key_ref and pending_state variants\"),",

      update_state_arms =
        let state_systems =
          systems |> std.array.filter family_has_state_update
        in
        (
          state_systems
          |> std.array.map (fun f =>
            let ksv = (family_key_state f).name in
            m%"
(Ref::%{f.variant}(key_ref), KeyState::%{ksv}(key_state)) => {
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
        ++ "\n(_, _) => smart_keymap::key::KeyEvents::no_events(),",

      key_output_arms =
        let out_systems =
          systems |> std.array.filter family_has_key_output
        in
        (
          out_systems
          |> std.array.map (fun f =>
            let ksv = (family_key_state f).name in
            "(Ref::%{f.variant}(r), KeyState::%{ksv}(ks)) => self.%{f.field}.key_output(r, ks),"
          )
          |> join
        )
        ++ "\n(_, _) => None,",

      key_state_from_family =
        systems
        |> std.array.map (fun f =>
          let ks = family_key_state f in
          m%"
impl From<%{ks.ty}> for KeyState {
    fn from(ks: %{ks.ty}) -> Self { KeyState::%{ks.name}(ks) }
}"%
        )
        |> join,

      pending_from_family =
        systems
        |> std.array.map (fun f =>
          let pty = family_pending_ty f in
          m%"
impl From<%{pty}> for PendingKeyState {
    fn from(pks: %{pty}) -> Self { PendingKeyState::%{f.variant}(pks) }
}"%
        )
        |> join,

      pending_mut_try_from =
        systems
        |> std.array.filter family_has_pending
        |> std.array.map (fun f =>
          let pty = family_pending_ty f in
          m%"
#[allow(unreachable_patterns)]
impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut %{pty} {
    type Error = ();
    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::%{f.variant}(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}"%
        )
        |> join,

      config_rust_expr =
        if config_systems == [] then
          "key_system::Config::new()"
        else
          let fields =
            config_systems
            |> std.array.map (fun f =>
              let c = family_config f in
              "%{f.field}: %{c.rust_expr},"
            )
            |> join
          in
          m%"key_system::Config {
%{fields}
}"%,

      system_rust_expr =
        if data_array_systems == [] then
          "key_system::System::new()"
        else
          let args =
            data_array_systems
            |> std.array.map (fun f =>
              let d = family_system_data f in
              "%{d.rust_expr},"
            )
            |> join
          in
          m%"key_system::System::new(
%{args}
)"%,

      chorded_pressed_indices_const =
        if has_chorded then
          "const CHORDED_MAX_PRESSED_INDICES: usize = crate::init::CHORDED_MAX_CHORD_SIZE * 2;\n"
        else
          "",

      key_state_variants =
        systems
        |> std.array.map (fun f =>
          let ks = family_key_state f in
          "/// [%{f.module}] key state.\n%{ks.name}(%{ks.ty}),"
        )
        |> join,

      module_doc =
        data
        |> match {
          'Array =>
            if is_full then
              "Full composite key system (generated; all registry families)."
            else
              "Per-keymap composite key system (generated; only families used by this keymap).",
          'Vec =>
            if is_full then
              "Full composite key system (generated; all registry families; vec-backed key data)."
            else
              "Per-keymap composite key system (generated; only families used by this keymap; vec-backed key data).",
        },

      system_derives =
        if is_vec then
          "#[derive(Debug, Clone, PartialEq)]"
        else
          "#[derive(Debug, Clone, Copy, PartialEq)]",

      rust_mod = m%"
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

        #[allow(unused_mut, unused_variables)]
        fn handle_event(
            &mut self,
            event: key::Event<Self::Event>,
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
%{enum_variants family_pending_ty}
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
    %{system_derives}
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

        #[allow(unused_variables)]
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
"%,

      # Key-data length consts for init (e.g. `const KEYBOARD: usize = N;`).
      # Profile-trimmed to active `systems`.
      init_consts_rust_stmts = fun key_data =>
        systems
        |> std.array.fold_left
          (fun acc f =>
            acc
            @ (
              f.system
              |> match {
                'System _ => [],
                'SystemWithData { data_lengths, .. } =>
                  data_lengths
                  |> std.array.map (fun { const_name, data_field } =>
                    let data = (key_data & { "%{data_field}" | default = [] })."%{data_field}" in
                    let len = data |> std.array.length |> std.to_string in
                    "    const %{const_name}: usize = %{len};"
                  ),
              }
            )
          )
          []
        |> std.string.join "\n",

      # Size / const-generic params for init (e.g. `LAYERED_LAYER_COUNT`).
      # Always emitted from the full family registry (v1 policy; not profile-trimmed).
      # `ctx = { json_keymap, key_codegen_values, smart_key }`.
      init_size_consts_rust_stmts = fun ctx =>
        family_list
        |> std.array.flat_map (fun f => f.init_params)
        |> std.array.map (fun p =>
          "    /// %{p.doc}\n    pub const %{p.const_name}: usize = %{std.to_string (p.value ctx)};"
        )
        |> std.string.join "\n\n",
    },

    # Public artefacts assembled from active fragments.
    config = {
      # JSON contract for keymap config fields (optional per family).
      Json = {
        automation | optional | smart_keymap.automation.config.Json,
        chorded | optional | smart_keymap.chorded.config.Json,
        layered | optional | smart_keymap.layered.config.Json,
        sticky | optional | smart_keymap.sticky.config.Json,
        tap_dance | optional | smart_keymap.tap_dance.config.Json,
        tap_hold | optional | smart_keymap.tap_hold.config.Json,
      },
      rust_expr = fragments.config_rust_expr,
    },
    system = {
      rust_expr = fragments.system_rust_expr,
      rust_mod = fragments.rust_mod,
      init_consts_rust_stmts = fragments.init_consts_rust_stmts,
      init_size_consts_rust_stmts = fragments.init_size_consts_rust_stmts,
    },
  },

  checks.composite_ref_wrap = {
    check_keyboard =
      let wrapped =
        composite.ref.wrap {
          module = "smart_keymap::key::keyboard",
          json = { KeyCode = 4 },
          rust_expr = "smart_keymap::key::keyboard::Ref::KeyCode(4)",
        }
      in
      {
        check_variant = {
          actual = wrapped.variant,
          expected = "Keyboard",
        },
        check_module = {
          actual = wrapped.module,
          expected = "key_system",
        },
        check_json = {
          actual = wrapped.json,
          expected = { Keyboard = { KeyCode = 4 } },
        },
        check_rust_expr = {
          actual = wrapped.rust_expr,
          expected = "key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4))",
        },
      },

    check_tap_hold =
      let wrapped =
        composite.ref.wrap {
          module = "smart_keymap::key::tap_hold",
          json = 0,
          rust_expr = "smart_keymap::key::tap_hold::Ref(0)",
        }
      in
      {
        check_variant = {
          actual = wrapped.variant,
          expected = "TapHold",
        },
        check_rust_expr = {
          actual = wrapped.rust_expr,
          expected = "key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(0))",
        },
      },
  },

  checks.composite_profile = {
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
          expected = std.record.length composite.families,
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

  checks.composite_full_emit =
    let rust_mod =
      (composite & composite.with_full_profile & { data = 'Array }).system.rust_mod
    in
    {
      check_module_doc_full = {
        actual = std.string.is_match "Full composite key system" rust_mod,
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
          |> std.array.all (fun needle => std.string.is_match needle rust_mod),
        expected = true,
      },
      check_concrete_system_new = {
        actual = std.string.is_match "pub const fn new" rust_mod,
        expected = true,
      },
      check_array_keyboard_system = {
        actual = std.string.is_match "\\[smart_keymap::key::keyboard::Key; crate::init::KEYBOARD\\]" rust_mod,
        expected = true,
      },
      check_no_keys_trait = {
        actual = std.string.is_match "trait Keys" rust_mod,
        expected = false,
      },
    },

  checks.composite_full_emit_vec =
    let rust_mod =
      (composite & composite.with_full_profile & { data = 'Vec }).system.rust_mod
    in
    {
      check_module_doc_full = {
        actual = std.string.is_match "Full composite key system" rust_mod,
        expected = true,
      },
      check_vec_storage_doc = {
        actual = std.string.is_match "vec-backed key data" rust_mod,
        expected = true,
      },
      check_vec_keyboard_system = {
        actual =
          std.string.is_match
            "Vec<smart_keymap::key::keyboard::Key>"
            rust_mod,
        expected = true,
      },
      check_not_array_keyboard = {
        actual = std.string.is_match "\\[smart_keymap::key::keyboard::Key; crate::init::KEYBOARD\\]" rust_mod,
        expected = false,
      },
      check_system_not_copy = {
        actual = std.string.is_match "#\\[derive\\(Debug, Clone, PartialEq\\)\\]" rust_mod,
        expected = true,
      },
      check_serde_default_on_config = {
        actual = std.string.is_match "#\\[serde\\(default\\)\\]" rust_mod,
        expected = true,
      },
      check_concrete_system_new = {
        actual = std.string.is_match "pub const fn new" rust_mod,
        expected = true,
      },
      check_no_keys_trait = {
        actual = std.string.is_match "trait Keys" rust_mod,
        expected = false,
      },
    },
}
