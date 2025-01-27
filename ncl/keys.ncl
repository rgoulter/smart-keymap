let key_extensions = {
  keyboard
    = fun K =>
        (import "hid-usage-keyboard.ncl")
        |> std.record.map_values
             (fun kc =>
                kc |> match {
                  _ if 0xE0 <= kc && kc <= 0xE7 => {
                    modifiers = kc |> match {
                      0xE0 => { left_ctrl = true },
                      0xE1 => { left_shift = true },
                      0xE2 => { left_alt = true },
                      0xE3 => { left_gui = true },
                      0xE4 => { right_ctrl = true },
                      0xE5 => { right_shift = true },
                      0xE6 => { right_alt = true },
                      0xE7 => { right_gui = true },
                    },
                  },

                  kc => { key_code = kc },
                }),

  tap_hold
    = fun K => {
        hold
        | doc "creates a hold key modifier"
        = fun key => { hold = key },

        H_LCtrl = hold K.LeftCtrl,
        H_LShift = hold K.LeftShift,
        H_LAlt = hold K.LeftAlt,
        H_LGUI = hold K.LeftGUI,
        H_RCtrl = hold K.RightCtrl,
        H_RShift = hold K.RightShift,
        H_RAlt = hold K.RightAlt,
        H_RGUI = hold K.RightGUI,
      },

  layered
    = fun K => {
        layer_mod = {
          hold = fun layer_num => {
            layer_modifier = { hold = layer_num }
          },
        },

        # Layer Transparency
        TTTT = null,
      },

  shifted
    = fun K =>
        {
          Exclaim = K.N1,
          At = K.N2,
          Hash = K.N3,
          Dollar = K.N4,
          Percent = K.N5,
          Caret = K.N6, # ^
          Ampersand = K.N7,
          Asterisk = K.N8,
          LeftParen = K.N9,
          RightParen = K.N0,

          LeftCurlyBracket = K.LeftBracket,
          RightCurlyBracket = K.RightBracket,

          Tilde = K.Grave, # ~

          Plus = K.Equals,
          Underscore = K.Minus,

          Question = K.Slash,
          Pipe = K.Backslash,

          Colon = K.Semicolon,
          DoubleQuote = K.Quote,

          LeftAngleBracket = K.Comma,
          RightAngleBracket = K.Dot,
        } |> std.record.map_values (fun k => k & K.LeftShift),

  aliases
    = fun K =>
        {
          Enter = K.Return,
          Circumflex = K.Caret, # ^
          LeftBrace = K.LeftCurlyBracket, # {
          RightBrace = K.RightCurlyBracket,
        },

  abbreviations
    = fun K =>
        {
          XXXX = K.NO,

          RET  = K.Return,
          ESC  = K.Escape,
          BSPC = K.Backspace,
          TAB  = K.Tab,
          SPC  = K.Space,

          ENT  = K.Enter,

          EXCL = K.Exclaim,
          AT   = K.At,
          HASH = K.Hash,
          DLR  = K.Dollar,
          PCT  = K.Percent,
          CARE = K.Caret, # ^
          AMP  = K.Ampersand,
          ASTR = K.Asterisk,
          LPRN = K.LeftParen,
          RPRN = K.RightParen,

          PERC = K.Percent,
          CIRC = K.Circumflex, # ^

          LBRK = K.LeftBracket,  # [
          LCBR = K.LeftCurlyBracket,  # {
          LBRC = K.LeftBrace,  # {
          RBRK = K.RightBracket,
          RCBR = K.RightCurlyBracket,
          RBRC = K.RightBrace,

          GRV = K.Grave,   # `
          TILD = K.Tilde,  # ~

          MINS = K.Minus,
          UNDS = K.Underscore,
          EQLS = K.Equals,
          PLUS = K.Plus,

          SLSH = K.Slash,
          QUES = K.Question,
          BSLS = K.Backslash,
          PIPE = K.Pipe,

          SCLN = K.Semicolon,
          COLN = K.Colon,
          QUOT = K.Quote,
          DQUO = K.DoubleQuote,

          COMM = K.Comma,
          LABR = K.LeftAngleBracket,
          DOT  = K.Dot,
          RABR = K.RightAngleBracket,

          CAPS = K.CapsLock,

          PSCR = K.PrintScreen,
          SCRL = K.ScrollLock,
          PAUS = K.Pause,

          INS  = K.Insert,
          HOME = K.Home,
          PGUP = K.PageUp,
          DEL  = K.Delete,
          END  = K.End,
          PGDN = K.PageDown,

          RGHT = K.Right,
          LEFT = K.Left,
          DOWN = K.Down,
          UP   = K.Up,

          NPNL = K.NumLock,
          NPSL = K.NPSlash,
          NPST = K.NPStar,
          NPMN = K.NPMinus,
          NPPL = K.NPPlus,
          NPEN = K.NPEnter,
          NPDT = K.NPDot,

          LCTL = K.LeftCtrl,
          LSFT = K.LeftShift,
          LALT = K.LeftAlt,
          LGUI = K.LeftGUI,
          RCTL = K.RightCtrl,
          RSFT = K.RightShift,
          RALT = K.RightAlt,
          RGUI = K.RightGUI,

          APPN = K.Application,

          NUSH = K.NonUSHash,
          NUSB = K.NonUSBackslash,
        },

  literals
    = fun K =>
        {
          "1" = K.N1,
          "2" = K.N2,
          "3" = K.N3,
          "4" = K.N4,
          "5" = K.N5,
          "6" = K.N6,
          "7" = K.N7,
          "8" = K.N8,
          "9" = K.N9,
          "0" = K.N0,

          "!" = K.Exclaim,
          "@" = K.At,
          "#" = K.Hash,
          "$" = K.Dollar,
          "%" = K.Percent,
          "^" = K.Caret,
          "&" = K.Ampersand,
          "*" = K.Asterisk,
          "(" = K.LeftParen,
          ")" = K.RightParen,

          "["  = K.LeftBracket,
          "{" = K.LeftCurlyBracket,
          "]"  = K.RightBracket,
          "}" = K.RightCurlyBracket,

          "`" = K.Grave,
          "~" = K.Tilde,

          "-"  = K.Minus,
          "_" = K.Underscore,
          "="  = K.Equals,
          "+" = K.Plus,

          "/"  = K.Slash,
          "?" = K.Question,
          "\\" = K.Backslash,
          "|" = K.Pipe,

          ";"  = K.Semicolon,
          ":"  = K.Colon,
          "'"  = K.Quote,
          "\""  = K.DoubleQuote,

          ","  = K.Comma,
          "<" = K.LeftAngleBracket,
          "."  = K.Dot,
          ">" = K.RightAngleBracket,
        },
} in

let extend_keys = fun K key_module => K & key_module K in

std.array.fold_left
  extend_keys
  {}
  [
    key_extensions.keyboard,
    key_extensions.tap_hold,
    key_extensions.layered,
    key_extensions.shifted,
    key_extensions.aliases,
    key_extensions.abbreviations,
    key_extensions.literals,
  ]
