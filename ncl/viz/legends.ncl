# Per-key legend projection. Match is shallow (field presence, then small maps).
#
# Firmware JSON is lossy; this walks keymap.ncl key trees.
let usage = import "hid-usage-keyboard.ncl" in
{
  code_to_name =
    std.record.fields usage
    |> std.array.fold_left
      (fun acc name => acc & { "%{std.record.get name usage}" = name })
      {},

  shorts = {
    Escape = "Esc",
    Backspace = "Bksp",
    Return = "Ent",
    Delete = "Del",
    Insert = "Ins",
    Space = "Spc",
    Tab = "Tab",
    CapsLock = "Caps",
    LeftCtrl = "Ctl",
    LeftShift = "Sft",
    LeftAlt = "Alt",
    LeftGUI = "GUI",
    RightCtrl = "Ctl",
    RightShift = "Sft",
    RightAlt = "Alt",
    RightGUI = "GUI",
    PageUp = "PgUp",
    PageDown = "PgDn",
    PrintScreen = "PSc",
    ScrollLock = "ScLk",
    Pause = "Paus",
    Application = "App",
    LeftBracket = "[",
    RightBracket = "]",
    Backslash = "\\",
    Semicolon = ";",
    Quote = "'",
    Grave = "`",
    Comma = ",",
    Dot = ".",
    Slash = "/",
    Minus = "-",
    Equals = "=",
    NonUSHash = "#",
    NonUSBackslash = "\\",
  },

  shifted_glyphs = {
    "30" = "!",
    "31" = "@",
    "32" = "#",
    "33" = "$",
    "34" = "%",
    "35" = "^",
    "36" = "&",
    "37" = "*",
    "38" = "(",
    "39" = ")",
    "45" = "_",
    "46" = "+",
    "47" = "{",
    "48" = "}",
    "49" = "|",
    "51" = ":",
    "52" = "\"",
    "53" = "~",
    "54" = "<",
    "55" = ">",
    "56" = "?",
  },

  digit_from_n = {
    N0 = "0",
    N1 = "1",
    N2 = "2",
    N3 = "3",
    N4 = "4",
    N5 = "5",
    N6 = "6",
    N7 = "7",
    N8 = "8",
    N9 = "9",
  },

  usage_label = fun kc =>
    let key = "%{kc}" in
    if std.record.has_field key code_to_name then
      let name = std.record.get key code_to_name in
      if std.record.has_field name shorts then
        std.record.get name shorts
      else if std.record.has_field name digit_from_n then
        std.record.get name digit_from_n
      else if std.string.length name == 1 then
        name
      else if std.string.is_match "^F[0-9]+$" name then
        name
      else
        name
    else
      "0x%{kc}",

  mods_label = fun m =>
    let flags = [
      { on = std.record.has_field "left_ctrl" m && m.left_ctrl, s = "Ctl" },
      { on = std.record.has_field "right_ctrl" m && m.right_ctrl, s = "Ctl" },
      { on = std.record.has_field "left_shift" m && m.left_shift, s = "Sft" },
      { on = std.record.has_field "right_shift" m && m.right_shift, s = "Sft" },
      { on = std.record.has_field "left_alt" m && m.left_alt, s = "Alt" },
      { on = std.record.has_field "right_alt" m && m.right_alt, s = "Alt" },
      { on = std.record.has_field "left_gui" m && m.left_gui, s = "GUI" },
      { on = std.record.has_field "right_gui" m && m.right_gui, s = "GUI" },
    ]
    in
    flags
    |> std.array.filter (fun f => f.on)
    |> std.array.map (fun f => f.s)
    |> std.string.join "+",

  consumer_shorts = {
    "233" = "Vol+",
    "234" = "Vol-",
    "226" = "Mute",
    "205" = "Play",
    "183" = "Stop",
    "181" = "Next",
    "182" = "Prev",
  },

  consumer_label = fun code =>
    let key = "%{code}" in
    if std.record.has_field key consumer_shorts then
      std.record.get key consumer_shorts
    else
      "C%{code}",

  mouse_label = fun mouse =>
    mouse
    |> match {
      { Button = n } => "M%{n}",
      s if std.is_string s => s,
      _ => "Ms",
    },

  callback_label = fun cb =>
    cb
    |> match {
      "ResetToBootloader" => "BOOT",
      "Reset" => "RST",
      { Bluetooth = _ } => "BT",
      { Custom = _ } => "Fn",
      _ => "Cb",
    },

  layer_mod_label = fun lm =>
    if std.record.has_field "hold" lm then
      "H(L%{lm.hold})"
    else if std.record.has_field "toggle" lm then
      "Tg(L%{lm.toggle})"
    else if std.record.has_field "sticky" lm then
      "St(L%{lm.sticky})"
    else if std.record.has_field "lock" lm then
      if std.is_number lm.lock then
        "Lk(L%{lm.lock})"
      else
        "Lk"
    else if std.record.has_field "default_" lm then
      "Df(L%{lm.default_})"
    else if std.record.has_field "set_active_layers_to" lm then
      "LY"
    else
      "LM",

  # C/S/A/G prefix for extra modifiers. Shift used to pick a glyph is omitted.
  mod_letters = fun m skip_shift =>
    [
      {
        on =
          (std.record.has_field "left_ctrl" m && m.left_ctrl)
          || (std.record.has_field "right_ctrl" m && m.right_ctrl),
        s = "C",
      },
      {
        on =
          !skip_shift
          && (
            (std.record.has_field "left_shift" m && m.left_shift)
            || (std.record.has_field "right_shift" m && m.right_shift)
          ),
        s = "S",
      },
      {
        on =
          (std.record.has_field "left_alt" m && m.left_alt)
          || (std.record.has_field "right_alt" m && m.right_alt),
        s = "A",
      },
      {
        on =
          (std.record.has_field "left_gui" m && m.left_gui)
          || (std.record.has_field "right_gui" m && m.right_gui),
        s = "G",
      },
    ]
    |> std.array.filter (fun f => f.on)
    |> std.array.map (fun f => f.s)
    |> std.string.join "",

  wrap_mods = fun letters core =>
    if letters == "" then
      core
    else
      "%{letters}(%{core})",

  # XXXX / keys.NO: a no-output key, used as "this is the held layer-mod key".
  is_xxxx = fun k =>
    std.is_record k
    && (
      k == {}
      || (
        std.record.has_field "key_code" k
        && k.key_code == 0
        && std.array.length (std.record.fields k) == 1
      )
    ),

  is_transparent = fun k => k == null,

  # Label a key with wrappers already stripped (no hold / layered).
  label = fun k =>
    if k == null then
      ""
    else if !std.is_record k then
      "?"
    else if is_xxxx k then
      ""
    else if std.record.has_field "key_code" k then
      let shifted =
        std.record.has_field "modifiers" k
        && std.record.has_field "left_shift" k.modifiers
        && k.modifiers.left_shift
      in
      let gk = "%{k.key_code}" in
      let used_glyph = shifted && std.record.has_field gk shifted_glyphs in
      let core =
        if used_glyph then
          std.record.get gk shifted_glyphs
        else if k.key_code == 0 then
          ""
        else
          usage_label k.key_code
      in
      if core == "" then
        ""
      else if std.record.has_field "modifiers" k then
        wrap_mods (mod_letters k.modifiers used_glyph) core
      else
        core
    else if std.record.has_field "consumer_code" k then
      consumer_label k.consumer_code
    else if std.record.has_field "mouse" k then
      mouse_label k.mouse
    else if std.record.has_field "keymap_callback" k then
      callback_label k.keymap_callback
    else if std.record.has_field "layer_modifier" k then
      layer_mod_label k.layer_modifier
    else if std.record.has_field "modifiers" k then
      mods_label k.modifiers
    else if k == {} then
      ""
    else
      "?",

  strip = fun k field =>
    if std.is_record k && std.record.has_field field k then
      std.record.remove field k
    else
      k,

  overlays_of = fun k =>
    if std.is_record k && std.record.has_field "layered" k then
      k.layered
      |> std.array.map (fun slot => if slot == null then "" else label (strip (strip slot "hold") "tap_hold_profile"))
    else
      [],

  empty = { center = "", hold = "", tl = "", tr = "", bl = "" },

  # Slot legends for one key (base tap/hold + optional layered overlays).
  of_key = fun k =>
    if k == null then
      empty
    else
      let overlays = overlays_of k in
      let k = strip k "layered" in
      let k = strip k "tap_hold_profile" in
      let hold_txt =
        if std.is_record k && std.record.has_field "hold" k then
          label k.hold
        else
          ""
      in
      let tap =
        if std.is_record k && std.record.has_field "hold" k then
          strip k "hold"
        else
          k
      in
      let at = fun i =>
        if i < std.array.length overlays then std.array.at i overlays else ""
      in
      {
        center = label tap,
        hold = hold_txt,
        tl = at 0,
        tr = at 1,
        bl = at 2,
      },

  of_layer_key = fun k =>
    if k == null then
      empty
    else
      let k = strip k "layered" in
      let k = strip k "tap_hold_profile" in
      let hold_txt =
        if std.is_record k && std.record.has_field "hold" k then
          label k.hold
        else
          ""
      in
      let tap =
        if std.is_record k && std.record.has_field "hold" k then
          strip k "hold"
        else
          k
      in
      {
        center = label tap,
        hold = hold_txt,
        tl = "",
        tr = "",
        bl = "",
      },

  emphasis_of = fun k =>
    if is_transparent k then
      'Transparent
    else if is_xxxx k then
      'Unused
    else
      'Normal,

  # Destination layer index for a hold-layer-mod key (tap or hold half).
  layer_hold_index = fun k =>
    if !std.is_record k then
      null
    else if std.record.has_field "layer_modifier" k
    && std.record.has_field "hold" k.layer_modifier then
      k.layer_modifier.hold
    else if std.record.has_field "hold" k then
      layer_hold_index k.hold
    else
      null,

  checks =
    let K = import "keys.ncl" in
    {
      check_letter = {
        actual = label K.A,
        expected = "A",
      },
      check_esc = {
        actual = label K.Escape,
        expected = "Esc",
      },
      check_digit = {
        actual = label K.N1,
        expected = "1",
      },
      check_shifted = {
        actual = label K.Exclaim,
        expected = "!",
      },
      check_null = {
        actual = label null,
        expected = "",
      },
      check_boot = {
        actual = label K.reset_to_bootloader,
        expected = "BOOT",
      },
      check_tap_hold =
        let k = K.A & K.hold K.LeftCtrl in
        {
          actual = of_key k,
          expected = {
            center = "A",
            hold = "Ctl",
            tl = "",
            tr = "",
            bl = "",
          },
        },
      check_layer_mod = {
        actual = label (K.layer_mod.hold 2),
        expected = "H(L2)",
      },
      check_gui_pgup = {
        actual = label (K.LeftGUI & K.PageUp),
        expected = "G(PgUp)",
      },
      check_xxxx = {
        actual = { blank = label K.NO, unused = is_xxxx K.NO, live = is_xxxx K.A },
        expected = { blank = "", unused = true, live = false },
      },
      check_layer_hold_index = {
        actual = {
          hold = layer_hold_index (K.layer_mod.hold 2),
          tap_hold = layer_hold_index (K.Tab & K.hold (K.layer_mod.hold 8)),
          letter = layer_hold_index K.A,
        },
        expected = { hold = 2, tap_hold = 8, letter = null },
      },
    },
}
