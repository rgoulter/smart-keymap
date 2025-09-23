{
  key_extensions = {
    keyboard =
      (import "hid-usage-keyboard.ncl")
      |> std.record.map_values (fun kc =>
        kc
        |> match {
          # 0xE0 .. 0xE7
          _ if 224 <= kc && kc <= 231 =>
            {
              modifiers =
                kc
                |> match {
                  224 => { left_ctrl = true },
                  225 => { left_shift = true },
                  226 => { left_alt = true },
                  227 => { left_gui = true },
                  228 => { right_ctrl = true },
                  229 => { right_shift = true },
                  230 => { right_alt = true },
                  231 => { right_gui = true },
                },
            },
          kc => { key_code = kc },
        }
      ),

    abbreviations = {
      keys,

      XXXX = keys.NO,

      RET = keys.Return,
      ESC = keys.Escape,
      BSPC = keys.Backspace,
      TAB = keys.Tab,
      SPC = keys.Space,

      ENT = keys.Enter,

      EXCL = keys.Exclaim,
      AT = keys.At,
      HASH = keys.Hash,
      DLR = keys.Dollar,
      PCT = keys.Percent,
      CARE = keys.Caret, # ^
      AMP = keys.Ampersand,
      ASTR = keys.Asterisk,
      LPRN = keys.LeftParen,
      RPRN = keys.RightParen,

      PERC = keys.Percent,
      CIRC = keys.Circumflex, # ^

      LBRK = keys.LeftBracket, # [
      LCBR = keys.LeftCurlyBracket, # {
      LBRC = keys.LeftBrace, # {
      RBRK = keys.RightBracket,
      RCBR = keys.RightCurlyBracket,
      RBRC = keys.RightBrace,

      GRV = keys.Grave, # `
      TILD = keys.Tilde, # ~

      MINS = keys.Minus,
      UNDS = keys.Underscore,
      EQLS = keys.Equals,
      PLUS = keys.Plus,

      SLSH = keys.Slash,
      QUES = keys.Question,
      BSLS = keys.Backslash,
      PIPE = keys.Pipe,

      SCLN = keys.Semicolon,
      COLN = keys.Colon,
      QUOT = keys.Quote,
      DQUO = keys.DoubleQuote,

      COMM = keys.Comma,
      LABR = keys.LeftAngleBracket,
      DOT = keys.Dot,
      RABR = keys.RightAngleBracket,

      CAPS = keys.CapsLock,

      PSCR = keys.PrintScreen,
      SCRL = keys.ScrollLock,
      PAUS = keys.Pause,

      SLCK = keys.ScrollLock,

      INS = keys.Insert,
      HOME = keys.Home,
      PGUP = keys.PageUp,
      DEL = keys.Delete,
      END = keys.End,
      PGDN = keys.PageDown,

      RGHT = keys.Right,
      LEFT = keys.Left,
      DOWN = keys.Down,
      UP = keys.Up,

      NPNL = keys.NumLock,
      NPSL = keys.NPSlash,
      NPST = keys.NPStar,
      NPMN = keys.NPMinus,
      NPPL = keys.NPPlus,
      NPEN = keys.NPEnter,
      NPDT = keys.NPDot,

      LCTL = keys.LeftCtrl,
      LSFT = keys.LeftShift,
      LALT = keys.LeftAlt,
      LGUI = keys.LeftGUI,
      RCTL = keys.RightCtrl,
      RSFT = keys.RightShift,
      RALT = keys.RightAlt,
      RGUI = keys.RightGUI,

      APPN = keys.Application,

      NUSH = keys.NonUSHash,
      NUSB = keys.NonUSBackslash,

      RST = keys.reset,
      BOOT = keys.reset_to_bootloader,

      CWTG = keys.caps_word.toggle,
    },

    aliases = {
      keys,

      Enter = keys.Return,
      Circumflex = keys.Caret, # ^
      LeftBrace = keys.LeftCurlyBracket, # {
      RightBrace = keys.RightCurlyBracket,
    },

    literals = {
      keys,

      "1" = keys.N1,
      "2" = keys.N2,
      "3" = keys.N3,
      "4" = keys.N4,
      "5" = keys.N5,
      "6" = keys.N6,
      "7" = keys.N7,
      "8" = keys.N8,
      "9" = keys.N9,
      "0" = keys.N0,

      "!" = keys.Exclaim,
      "@" = keys.At,
      "#" = keys.Hash,
      "$" = keys.Dollar,
      "%" = keys.Percent,
      "^" = keys.Caret,
      "&" = keys.Ampersand,
      "*" = keys.Asterisk,
      "(" = keys.LeftParen,
      ")" = keys.RightParen,

      "[" = keys.LeftBracket,
      "{" = keys.LeftCurlyBracket,
      "]" = keys.RightBracket,
      "}" = keys.RightCurlyBracket,

      "`" = keys.Grave,
      "~" = keys.Tilde,

      "-" = keys.Minus,
      "_" = keys.Underscore,
      "=" = keys.Equals,
      "+" = keys.Plus,

      "/" = keys.Slash,
      "?" = keys.Question,
      "\\" = keys.Backslash,
      "|" = keys.Pipe,

      ";" = keys.Semicolon,
      ":" = keys.Colon,
      "'" = keys.Quote,
      "\"" = keys.DoubleQuote,

      "," = keys.Comma,
      "<" = keys.LeftAngleBracket,
      "." = keys.Dot,
      ">" = keys.RightAngleBracket,
    },

    shifted = {
      keys,

      Exclaim = keys.N1 & keys.LeftShift,
      At = keys.N2 & keys.LeftShift,
      Hash = keys.N3 & keys.LeftShift,
      Dollar = keys.N4 & keys.LeftShift,
      Percent = keys.N5 & keys.LeftShift,
      Caret = keys.N6 & keys.LeftShift, # ^
      Ampersand = keys.N7 & keys.LeftShift,
      Asterisk = keys.N8 & keys.LeftShift,
      LeftParen = keys.N9 & keys.LeftShift,
      RightParen = keys.N0 & keys.LeftShift,

      LeftCurlyBracket = keys.LeftBracket & keys.LeftShift,
      RightCurlyBracket = keys.RightBracket & keys.LeftShift,

      Tilde = keys.Grave & keys.LeftShift, # ~

      Plus = keys.Equals & keys.LeftShift,
      Underscore = keys.Minus & keys.LeftShift,

      Question = keys.Slash & keys.LeftShift,
      Pipe = keys.Backslash & keys.LeftShift,

      Colon = keys.Semicolon & keys.LeftShift,
      DoubleQuote = keys.Quote & keys.LeftShift,

      LeftAngleBracket = keys.Comma & keys.LeftShift,
      RightAngleBracket = keys.Dot & keys.LeftShift,
    },
  },
}
