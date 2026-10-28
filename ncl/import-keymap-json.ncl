# Import a QMK Configurator-style `keymap.json` (a `layout` + `layers` array
#  of QMK keycode strings) into smart-keymap's Nickel authoring shape.
#
# Usage:
#   let { to_smart_keymap, .. } = import "import-keymap-json.ncl" in
#   import "my-qmk-keymap.json" |> to_smart_keymap board_index_order
#
# `board_index_order` remaps physical key positions: QMK's per-keyboard
#  `LAYOUT` macro order rarely matches the order a smart-keymap board scans
#  its keys in, so callers give the QMK layout index for each smart-keymap
#  key position (`board_index_order.[i]` is the QMK index for the key at
#  smart-keymap position `i`).
#
# Unsupported QMK features (tap-hold, tap dance, macros, and any top-level
#  fields this module doesn't recognise, e.g. combos/overrides) are dropped
#  rather than guessed at, and reported in the `warnings` array.
let K = import "keys.ncl" in

let qmk = {
  # QMK keycode (with `KC_` prefix, as emitted by the Configurator) → the
  #  field of `keys.ncl` that means the same thing. Kept explicit rather
  #  than derived from smart-keymap's own abbreviations, since a few QMK
  #  short names collide with different smart-keymap abbreviations
  #  (e.g. QMK's `KC_LBRC` is `[`, smart-keymap's `LBRC` is `{`).
  keycode_to_key_field = {
    KC_A = "A",
    KC_B = "B",
    KC_C = "C",
    KC_D = "D",
    KC_E = "E",
    KC_F = "F",
    KC_G = "G",
    KC_H = "H",
    KC_I = "I",
    KC_J = "J",
    KC_K = "K",
    KC_L = "L",
    KC_M = "M",
    KC_N = "N",
    KC_O = "O",
    KC_P = "P",
    KC_Q = "Q",
    KC_R = "R",
    KC_S = "S",
    KC_T = "T",
    KC_U = "U",
    KC_V = "V",
    KC_W = "W",
    KC_X = "X",
    KC_Y = "Y",
    KC_Z = "Z",

    KC_1 = "N1",
    KC_2 = "N2",
    KC_3 = "N3",
    KC_4 = "N4",
    KC_5 = "N5",
    KC_6 = "N6",
    KC_7 = "N7",
    KC_8 = "N8",
    KC_9 = "N9",
    KC_0 = "N0",

    KC_ENT = "Return",
    KC_ENTER = "Return",
    KC_ESC = "Escape",
    KC_ESCAPE = "Escape",
    KC_BSPC = "Backspace",
    KC_BACKSPACE = "Backspace",
    KC_TAB = "Tab",
    KC_SPC = "Space",
    KC_SPACE = "Space",

    KC_MINS = "Minus",
    KC_MINUS = "Minus",
    KC_EQL = "Equals",
    KC_EQUAL = "Equals",
    KC_LBRC = "LeftBracket",
    KC_LEFT_BRACKET = "LeftBracket",
    KC_RBRC = "RightBracket",
    KC_RIGHT_BRACKET = "RightBracket",
    KC_BSLS = "Backslash",
    KC_BACKSLASH = "Backslash",
    KC_SCLN = "Semicolon",
    KC_SEMICOLON = "Semicolon",
    KC_QUOT = "Quote",
    KC_QUOTE = "Quote",
    KC_GRV = "Grave",
    KC_GRAVE = "Grave",
    KC_COMM = "Comma",
    KC_COMMA = "Comma",
    KC_DOT = "Dot",
    KC_PERIOD = "Dot",
    KC_SLSH = "Slash",
    KC_SLASH = "Slash",
    KC_CAPS = "CapsLock",
    KC_CAPSLOCK = "CapsLock",

    KC_F1 = "F1",
    KC_F2 = "F2",
    KC_F3 = "F3",
    KC_F4 = "F4",
    KC_F5 = "F5",
    KC_F6 = "F6",
    KC_F7 = "F7",
    KC_F8 = "F8",
    KC_F9 = "F9",
    KC_F10 = "F10",
    KC_F11 = "F11",
    KC_F12 = "F12",

    KC_PSCR = "PrintScreen",
    KC_PRINT_SCREEN = "PrintScreen",
    KC_SCRL = "ScrollLock",
    KC_SCROLL_LOCK = "ScrollLock",
    KC_PAUS = "Pause",
    KC_PAUSE = "Pause",

    KC_INS = "Insert",
    KC_INSERT = "Insert",
    KC_HOME = "Home",
    KC_PGUP = "PageUp",
    KC_PAGE_UP = "PageUp",
    KC_DEL = "Delete",
    KC_DELETE = "Delete",
    KC_END = "End",
    KC_PGDN = "PageDown",
    KC_PAGE_DOWN = "PageDown",

    KC_RGHT = "Right",
    KC_RIGHT = "Right",
    KC_LEFT = "Left",
    KC_DOWN = "Down",
    KC_UP = "Up",

    KC_LCTL = "LeftCtrl",
    KC_LEFT_CTRL = "LeftCtrl",
    KC_LSFT = "LeftShift",
    KC_LEFT_SHIFT = "LeftShift",
    KC_LALT = "LeftAlt",
    KC_LEFT_ALT = "LeftAlt",
    KC_LGUI = "LeftGUI",
    KC_LEFT_GUI = "LeftGUI",
    KC_RCTL = "RightCtrl",
    KC_RIGHT_CTRL = "RightCtrl",
    KC_RSFT = "RightShift",
    KC_RIGHT_SHIFT = "RightShift",
    KC_RALT = "RightAlt",
    KC_RIGHT_ALT = "RightAlt",
    KC_RGUI = "RightGUI",
    KC_RIGHT_GUI = "RightGUI",

    KC_APP = "Application",
    KC_APPLICATION = "Application",
    KC_NUHS = "NonUSHash",
    KC_NONUS_HASH = "NonUSHash",
    KC_NUBS = "NonUSBackslash",
    KC_NONUS_BACKSLASH = "NonUSBackslash",

    # Shifted symbols: QMK models these as distinct keycodes, smart-keymap
    #  as a base key held with shift (see `keyboard_shifted` extension).
    KC_EXLM = "Exclaim",
    KC_AT = "At",
    KC_HASH = "Hash",
    KC_DLR = "Dollar",
    KC_PERC = "Percent",
    KC_CIRC = "Caret",
    KC_AMPR = "Ampersand",
    KC_ASTR = "Asterisk",
    KC_LPRN = "LeftParen",
    KC_RPRN = "RightParen",
    KC_LCBR = "LeftCurlyBracket",
    KC_RCBR = "RightCurlyBracket",
    KC_TILD = "Tilde",
    KC_PLUS = "Plus",
    KC_UNDS = "Underscore",
    KC_QUES = "Question",
    KC_PIPE = "Pipe",
    KC_COLN = "Colon",
    KC_DQUO = "DoubleQuote",
    KC_LT = "LeftAngleBracket",
    KC_GT = "RightAngleBracket",
  },

  # Codes with no `keys.ncl` field lookup: literal/special-cased results.
  keycode_to_key = {
    KC_TRNS = K.TTTT,
    KC_TRANSPARENT = K.TTTT,
    "_______" = K.TTTT,

    KC_NO = K.XXXX,
    "XXXXXXX" = K.XXXX,

    QK_BOOT = K.BOOT,
    RESET = K.BOOT,
  },

  # Layer-switching functions: `NAME(number)`.
  layer_switch_fns = {
    MO = K.layer_mod.hold,
    TG = K.layer_mod.toggle,
    TO = K.layer_mod.set_default,
    DF = K.layer_mod.set_default,
    OSL = K.layer_mod.sticky,
  },

  layer_switch_match = fun code =>
    std.string.find "^([A-Z]+)\\(([0-9]+)\\)$" code
    |> match {
      { matched = _, groups = [fn_name, layer_num_str], .. } =>
        if std.record.has_field fn_name layer_switch_fns then
          'Some (std.record.get fn_name layer_switch_fns (std.string.to_number layer_num_str))
        else
          'None,
      _ => 'None,
    },

  # QMK features this import deliberately does not translate: tap-hold
  #  (mod-tap / layer-tap), tap dance, and one-shot/dynamic macros. These
  #  combine timing behaviour with the key/layer they wrap, and guessing a
  #  smart-keymap equivalent would silently change behaviour, so they're
  #  dropped with a warning instead.
  unsupported_patterns = [
    { regex = "_T\\(", label = "tap-hold" },
    { regex = "^MT\\(", label = "tap-hold" },
    { regex = "^LT\\(", label = "tap-hold (layer-tap)" },
    { regex = "^TT\\(", label = "tap-toggle (tap-hold-like)" },
    { regex = "^TD\\(", label = "tap dance" },
    { regex = "^DM_", label = "dynamic macro" },
    { regex = "^M\\(", label = "macro" },
    { regex = "^SS_", label = "macro" },
  ],

  unsupported_match = fun code =>
    unsupported_patterns
    |> std.array.filter (fun { regex, .. } => std.string.is_match regex code)
    |> match {
      [{ label, .. }, ..] => 'Some label,
      [] => 'None,
    },

  # Convert one QMK keycode string to `{ key, warning }`, where `warning`
  #  is `null` unless the keycode was dropped.
  convert_keycode = fun code =>
    if std.record.has_field code keycode_to_key then
      { key = std.record.get code keycode_to_key, warning = null }
    else if std.record.has_field code keycode_to_key_field then
      { key = std.record.get (std.record.get code keycode_to_key_field) K, warning = null }
    else
      layer_switch_match code
      |> match {
        'Some matched_key => { key = matched_key, warning = null },
        'None =>
          unsupported_match code
          |> match {
            'Some label =>
              { key = K.NO, warning = "dropped unsupported %{label} keycode: %{code}" },
            'None =>
              { key = K.NO, warning = "dropped unrecognised keycode: %{code}" },
          },
      },

  # Reorders one QMK layer's flat keycode array into smart-keymap's board
  #  order, converting each keycode on the way.
  convert_layer = fun board_index_order qmk_layer =>
    board_index_order
    |> std.array.map (fun qmk_index =>
      if qmk_index >= 0 && qmk_index < std.array.length qmk_layer then
        convert_keycode (std.array.at qmk_index qmk_layer)
      else
        {
          key = K.NO,
          warning = "board_index_order refers to out-of-range QMK layout index: %{std.to_string qmk_index}",
        }
    )
    |> (fun converted =>
      {
        keys = converted |> std.array.map (fun { key, .. } => key),
        warnings = converted |> std.array.map (fun { warning, .. } => warning) |> std.array.filter (fun w => w != null),
      }
    ),

  recognised_top_level_fields = ["version", "notes", "documentation", "author", "keyboard", "keymap", "layout", "layers"],

  top_level_warnings = fun qmk_json =>
    qmk_json
    |> std.record.fields
    |> std.array.filter (fun field => !(std.array.elem field recognised_top_level_fields))
    |> std.array.map (fun field => "dropped unsupported top-level feature: %{field}"),

  to_smart_keymap = fun board_index_order qmk_json =>
    let converted_layers = qmk_json.layers |> std.array.map (convert_layer board_index_order) in
    {
      layers = converted_layers |> std.array.map (fun { keys, .. } => keys),
      warnings =
        (top_level_warnings qmk_json)
        @ (converted_layers |> std.array.map (fun { warnings, .. } => warnings) |> std.array.flatten),
    },
}
in

{
  json_keymap = import "keymap.json",

  to_smart_keymap = qmk.to_smart_keymap,
}
