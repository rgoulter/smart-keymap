# Pass: named-layer prepare + pad
#
# After automation, before to_json_value:
#   1. Collect global named-layer names (alphabetical)
#   2. Lower `named_layers` → dense `layered` arrays
#   3. Resolve layer_mod name strings → 1-based indices
#   4. Pad every layered array to uniform length
#
# Public API used by json_keymap and inputs-to-json:
#   prepare_keys_named_layers, named_layer_indices_from_keys,
#   resolve_layer_modifier_names, pad_key_layered_arrays,
#   max_layered_length_accum, named_layer_names, numbered_layer_count, …
{
  layered_keys,
  keymap_ncl,

  # Pre-lowering: only `layered` (array) contributes to N (numbered slots).
  #
  # Keys with only `named_layers` return 0 here. Their M slots are counted via
  # named_layer_names_* and appended in lower_named_layers_on_key as:
  #   [null * N] ++ [named slots in global order].
  numbered_layered_length_of = fun k =>
    k
    |> match {
      { layered, .. } if keymap_ncl.layered.is_key k => std.array.length layered,
      _ => 0,
    },

  # Longest numbered layered array in a key tree (via map_accum for nesting).
  max_numbered_layered_length_accum = fun acc k =>
    let len = numbered_layered_length_of k in
    let acc = if len > acc then len else acc in
    let { acc, k = _ } =
      keymap_ncl.nullable_key.map_accum
        (fun acc k =>
          let len = numbered_layered_length_of k in
          let acc = if len > acc then len else acc in
          { include acc, include k }
        )
        acc
        k
    in
    acc,

  # Alias used by padding / checks for dense array length after named-layer lowering
  # (everything is array-form by then; pre-lowering this is intentionally numbered-only).
  max_layered_length_accum = max_numbered_layered_length_accum,

  # Collect named-layer field names into a record set { layer_name = true, ... }.
  # std.record.fields of the result is alphabetically sorted (Nickel field order).
  named_layer_names_from_key = fun k =>
    k
    |> match {
      { named_layers, .. } if keymap_ncl.layered.is_key k =>
        named_layers
        |> std.record.fields
        |> std.array.fold_left
          (fun names layer_name => names & { "%{layer_name}" = true })
          {},
      _ => {},
    },

  named_layer_names_accum = fun acc_names k =>
    let acc_names = acc_names & named_layer_names_from_key k in
    let { acc = acc_names, k = _ } =
      keymap_ncl.nullable_key.map_accum
        (fun acc_names k =>
          { acc = acc_names & named_layer_names_from_key k, include k }
        )
        acc_names
        k
    in
    acc_names,

  named_layer_names_from_keys = fun keys =>
    keys
    |> std.array.fold_left named_layer_names_accum {}
    |> std.record.fields,

  numbered_layer_count_from_keys = fun keys =>
    keys |> std.array.fold_left max_numbered_layered_length_accum 0,

  # Total dense layered array length = numbered slots + named slots (named after numbered).
  total_layered_length_from_keys = fun keys =>
    let numbered = numbered_layer_count_from_keys keys in
    let named = named_layer_names_from_keys keys in
    numbered + std.array.length named,

  named_layer_names
    | doc "Alphabetically sorted named layer names used in the keymap."
    = named_layer_names_from_keys layered_keys,

  numbered_layer_count
    | doc "Max numbered (`layered` array) length across the keymap."
    = numbered_layer_count_from_keys layered_keys,

  max_layered_length
    | doc "Dense layered array length after named layers are appended (numbered + named)."
    = numbered_layer_count + std.array.length named_layer_names,

  # 1-based layer indices for layer_mod.* (layered[i] <-> layer i+1).
  # Named layers start after numbered slots: first named layer -> numbered_layer_count + 1.
  named_layer_indices
    | doc "Record of named layer name -> 1-based layer index (for layer_mod)."
    =
      named_layer_names
      |> std.array.map_with_index (fun i layer_name => { "%{layer_name}" = numbered_layer_count + i + 1 })
      |> std.array.fold_left (&) {},

  # Pad or truncate a numbered `layered` array to exactly `target_len`.
  pad_or_slice_layered = fun target_len layered =>
    let len = std.array.length layered in
    if len >= target_len then
      std.array.slice 0 target_len layered
    else
      layered @ std.array.generate (std.function.const null) (target_len - len),

  # Look up each global named-layer name in `named_rec`; missing -> null (transparency).
  named_slots_from = fun named_names named_rec =>
    let filled =
      named_names
      |> std.array.fold_left
        (fun acc layer_name => acc & { "%{layer_name}" | default = null })
        named_rec
    in
    named_names |> std.array.map (fun layer_name => std.record.get layer_name filled),

  # Scope: one key record (no nested walk).
  # Merge layered (numbered) + named_layers into a dense layered array.
  # Named layers occupy indices [numbered_len, numbered_len + |names|).
  # Strips named_layers. Non-layered keys (and null) pass through unchanged.
  lower_named_layers_on_key = fun numbered_len named_names k =>
    k
    |> match {
      k if !(std.is_record k && keymap_ncl.layered.is_key k) => k,
      { layered = numbered, named_layers = named, ..base_key } =>
        base_key
        & {
          layered =
            pad_or_slice_layered numbered_len numbered
            @ named_slots_from named_names named,
        },
      { layered = numbered, ..base_key } =>
        base_key
        & {
          layered =
            pad_or_slice_layered numbered_len numbered
            @ named_slots_from named_names {},
        },
      { named_layers = named, ..base_key } =>
        base_key
        & {
          layered =
            std.array.generate (std.function.const null) numbered_len
            @ named_slots_from named_names named,
        },
    },

  # Scope: one key tree (self, then nested via map_accum).
  lower_named_layers_in_key_tree = fun numbered_len named_names k =>
    let lower_f = fun acc key =>
      let lowered = lower_named_layers_on_key numbered_len named_names key in
      { include acc, k = lowered }
    in
    let k = lower_named_layers_on_key numbered_len named_names k in
    let { acc = _, k } = keymap_ncl.nullable_key.map_accum lower_f null k in
    k,

  # Scope: array of key trees (pipeline pass). No-op when no named layers used.
  lower_named_layers_on_keys = fun keys =>
    let named_names = named_layer_names_from_keys keys in
    named_names
    |> match {
      [] => keys,
      _ =>
        let numbered_len = numbered_layer_count_from_keys keys in
        keys |> std.array.map (lower_named_layers_in_key_tree numbered_len named_names),
    },

  # name → 1-based layer index from a key list (same rules as named_layer_indices).
  named_layer_indices_from_keys = fun keys =>
    let numbered = numbered_layer_count_from_keys keys in
    named_layer_names_from_keys keys
    |> std.array.map_with_index (fun i layer_name => { "%{layer_name}" = numbered + i + 1 })
    |> std.array.fold_left (&) {},

  # Resolve a single layer index: numbers pass through; strings look up name_to_idx.
  resolve_layer_index = fun name_to_idx idx_or_name =>
    idx_or_name
    |> match {
      n if std.is_number n => n,
      name if std.is_string name =>
        if std.record.has_field name name_to_idx then
          std.record.get name name_to_idx
        else
          std.fail_with "unknown named layer: %{name}",
      _ => std.fail_with "expected layer index number or name string",
    },

  resolve_layer_indices = fun name_to_idx arr =>
    std.array.map (resolve_layer_index name_to_idx) arr,

  # Rewrite layer_modifier fields that use named-layer strings into numeric indices.
  resolve_layer_modifier_names = fun name_to_idx k =>
    k
    |> match {
      { layer_modifier = { default_ = d } } =>
        { layer_modifier = { default_ = resolve_layer_index name_to_idx d } },
      { layer_modifier = { hold = h }, modifiers = m } =>
        {
          layer_modifier = { hold = resolve_layer_index name_to_idx h },
          modifiers = m,
        },
      { layer_modifier = { hold = h } } =>
        { layer_modifier = { hold = resolve_layer_index name_to_idx h } },
      { layer_modifier = { sticky = s } } =>
        { layer_modifier = { sticky = resolve_layer_index name_to_idx s } },
      { layer_modifier = { toggle = t } } =>
        { layer_modifier = { toggle = resolve_layer_index name_to_idx t } },
      { layer_modifier = { set_active_layers_to = active, affected_layers = mask } } =>
        {
          layer_modifier = {
            set_active_layers_to = resolve_layer_indices name_to_idx active,
            affected_layers = resolve_layer_indices name_to_idx mask,
          },
        },
      { layer_modifier = { set_active_layers_to = active } } =>
        {
          layer_modifier = {
            set_active_layers_to = resolve_layer_indices name_to_idx active,
          },
        },
      _ => k,
    },

  # Resolve named-layer strings in layer_modifier fields (and nested keys).
  resolve_named_layer_refs_in_key = fun name_to_idx k =>
    let resolve_f = fun acc key =>
      { include acc, k = resolve_layer_modifier_names name_to_idx key }
    in
    let k = resolve_layer_modifier_names name_to_idx k in
    let { acc = _, k } = keymap_ncl.nullable_key.map_accum resolve_f null k in
    k,

  resolve_keys_named_layer_refs = fun name_to_idx keys =>
    keys |> std.array.map (resolve_named_layer_refs_in_key name_to_idx),

  # Lower named_layers to dense arrays, then resolve layer_mod name strings.
  # Indices for names are taken from the pre-lowered key list so they stay stable.
  prepare_keys_named_layers = fun keys =>
    let name_to_idx = named_layer_indices_from_keys keys in
    let keys = lower_named_layers_on_keys keys in
    resolve_keys_named_layer_refs name_to_idx keys,

  # Pad a layered key's `layered` array with null (transparency) up to `target_len`.
  # Expects post-lowering form (array `layered` only; no `named_layers`).
  pad_layered_key = fun target_len k =>
    k
    |> match {
      { named_layers = _, .. } =>
        std.fail_with "pad_layered_key expects array-form layered (lower named layers first)",
      { layered = numbered, ..base_key } =>
        base_key & { layered = pad_or_slice_layered target_len numbered },
      k => k,
    },

  # Pad every layered key in a key tree to `target_len` (self, then nested via map_accum).
  pad_key_layered_arrays = fun target_len k =>
    let pad_f = fun acc key =>
      let padded = pad_layered_key target_len key in
      { include acc, k = padded }
    in
    let k = pad_layered_key target_len k in
    let { acc = _, k } = keymap_ncl.nullable_key.map_accum pad_f null k in
    k,

  checks.check_max_layered_length =
    let K = import "keys.ncl" in
    {
      check_unequal_lengths_uses_longest = {
        actual =
          [
            K.A & { layered = [K.B] },
            K.C & { layered = [ null, K.D] },
          ]
          |> std.array.fold_left max_layered_length_accum 0,
        expected = 2,
      },

      check_first_key_not_layered = {
        actual =
          [
            K.layer_mod.hold 1,
            K.A & { layered = [ null, K.B] },
          ]
          |> std.array.fold_left max_layered_length_accum 0,
        expected = 2,
      },

      check_no_layered_keys = {
        actual = [K.A, K.B] |> std.array.fold_left max_layered_length_accum 0,
        expected = 0,
      },

      check_pad_shorter_layered_array = {
        actual =
          pad_layered_key 3 (K.A & { layered = [K.B] }),
        expected = K.A & { layered = [K.B, null, null ] },
      },

      check_json_pads_unequal_layered_lengths =
        let keys = [
          K.A & { layered = [K.B] },
          K.C & { layered = [ null, K.D] },
        ]
        in
        let layer_count = keys |> std.array.fold_left max_layered_length_accum 0 in
        {
          actual =
            keys
            |> std.array.map (pad_key_layered_arrays layer_count)
            |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              layered = [
                { key_code = 5 },
                null,
              ],
            },
            {
              base = { key_code = 6 },
              layered = [
                null,
                { key_code = 7 },
              ],
            },
          ],
        },
    },

  checks.check_named_layers =
    let K = import "keys.ncl" in
    {
      check_named_layer_names_sorted = {
        actual =
          named_layer_names_from_keys [
            K.A & { named_layers = { windows = K.B, linux = K.C } },
            K.D & { named_layers = { macos = K.E } },
          ],
        expected = ["linux", "macos", "windows"],
      },

      check_named_layer_indices_after_numbered =
        let keys = [
          K.A & { layered = [ null, K.B] },
          K.C & { named_layers = { windows = K.D, linux = K.E } },
        ]
        in
        let numbered = numbered_layer_count_from_keys keys in
        let names = named_layer_names_from_keys keys in
        {
          actual =
            names
            |> std.array.map_with_index (fun i layer_name => { "%{layer_name}" = numbered + i + 1 })
            |> std.array.fold_left (&) {},
          # numbered length 2 -> linux=3, windows=4
          expected = { linux = 3, windows = 4 },
        },

      check_total_length_numbered_plus_named = {
        actual =
          total_layered_length_from_keys [
            K.A & { layered = [K.B] },
            K.C & { named_layers = { nav = K.D } },
          ],
        expected = 2,
      },

      check_lower_named_only =
        let keys = [K.A & { named_layers = { fn = K.B, nav = K.C } }] in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # named sorted: fn, nav -> indices 0, 1
              layered = [
                { key_code = 5 },
                { key_code = 6 },
              ],
            },
          ],
        },

      check_lower_numbered_then_named =
        let keys = [
          K.A & { layered = [ null, K.B] },
          K.C & { named_layers = { windows = K.D } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # numbered [null, B] + named slot windows
              layered = [
                null,
                { key_code = 5 },
                null,
              ],
            },
            {
              base = { key_code = 6 },
              # numbered padding + windows = D
              layered = [
                null,
                null,
                { key_code = 7 },
              ],
            },
          ],
        },

      check_lower_partial_named_union =
        let keys = [
          K.A & { named_layers = { linux = K.B, windows = K.C } },
          K.D & { named_layers = { macos = K.E } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # sorted: linux, macos, windows
              layered = [
                { key_code = 5 },
                null,
                { key_code = 6 },
              ],
            },
            {
              base = { key_code = 7 },
              layered = [
                null,
                { key_code = 8 },
                null,
              ],
            },
          ],
        },

      # Numbered + named on the same key (composable via `&`).
      check_lower_both_on_same_key =
        let keys = [
          K.A
          & { layered = [ null, K.B] }
          & { named_layers = { fn = K.C } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # numbered [null, B] + named fn = C
              layered = [
                null,
                { key_code = 5 },
                { key_code = 6 },
              ],
            },
          ],
        },

      # Named-layer refs in layer_mod: exclusive switch by name + named_layers payload.
      # Names sort alphabetically: fn=1, nav=2, sym=3 when no numbered layers.
      check_prepare_named_layer_mod_refs =
        let names = ["fn", "nav", "sym"] in
        let keys = [
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = ["sym"],
            affected_layers = names,
          },
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = ["fn"],
            affected_layers = names,
          },
          { named_layers = { sym = K.A, fn = K.B, nav = K.C } },
        ]
        in
        {
          actual = prepare_keys_named_layers keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              SetActiveLayers = {
                layers = std.number.pow 2 3,
                mask =
                  (std.number.pow 2 1)
                  + (std.number.pow 2 2)
                  + (std.number.pow 2 3),
              },
            },
            {
              SetActiveLayers = {
                layers = std.number.pow 2 1,
                mask =
                  (std.number.pow 2 1)
                  + (std.number.pow 2 2)
                  + (std.number.pow 2 3),
              },
            },
            {
              base = { key_code = 0 },
              # alphabetical: fn, nav, sym
              layered = [
                { key_code = 5 },
                { key_code = 6 },
                { key_code = 4 },
              ],
            },
          ],
        },

      check_prepare_named_layer_mod_refs_with_numbered =
        # Numbered length 1 → named fn=2, nav=3, sym=4
        let names = ["fn", "nav", "sym"] in
        let keys = [
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = ["sym"],
            affected_layers = names,
          },
          K.D & { layered = [K.E] },
          { named_layers = { sym = K.A, fn = K.B, nav = K.C } },
        ]
        in
        {
          actual = prepare_keys_named_layers keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              SetActiveLayers = {
                layers = std.number.pow 2 4,
                mask =
                  (std.number.pow 2 2)
                  + (std.number.pow 2 3)
                  + (std.number.pow 2 4),
              },
            },
            {
              base = { key_code = 7 },
              layered = [
                { key_code = 8 },
                null,
                null,
                null,
              ],
            },
            {
              base = { key_code = 0 },
              layered = [
                null,
                { key_code = 5 },
                { key_code = 6 },
                { key_code = 4 },
              ],
            },
          ],
        },

      # Same prepare path via authoring sugar (semantic + set_semantic_variant_to).
      check_prepare_semantic_sugar =
        let names = ["fn", "nav", "sym"] in
        let keys = [
          K.layer_mod.set_semantic_variant_to names "sym",
          K.layer_mod.set_semantic_variant_to names "fn",
          K.semantic { sym = K.A, fn = K.B, nav = K.C },
        ]
        in
        {
          actual = prepare_keys_named_layers keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              SetActiveLayers = {
                layers = std.number.pow 2 3,
                mask =
                  (std.number.pow 2 1)
                  + (std.number.pow 2 2)
                  + (std.number.pow 2 3),
              },
            },
            {
              SetActiveLayers = {
                layers = std.number.pow 2 1,
                mask =
                  (std.number.pow 2 1)
                  + (std.number.pow 2 2)
                  + (std.number.pow 2 3),
              },
            },
            {
              base = { key_code = 0 },
              layered = [
                { key_code = 5 },
                { key_code = 6 },
                { key_code = 4 },
              ],
            },
          ],
        },
    },
}
