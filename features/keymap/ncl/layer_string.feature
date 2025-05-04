Feature: Layer String

  Layers in `keymap.ncl` can be defined using strings.

  Example: keymap.ncl with layer string

    `keymap.ncl` supports defining each layer in the keymap
     with a string.

    This is simpler than the equivalent nickel array
     using keys such as `K.A`, `K.B`, etc.

    Each whitespace-delimited substring is then used
     as a field to lookup the key from `keys.ncl`.

    For example:

    * a keymap.ncl:
      """
      {
        layers = [
          m%"
            Q W E R T Y
          "%,
        ],
      }
      """

  Example: keymap.ncl with layer string, with custom key definitions

    Often you'll want to be able to use custom keys, even
     in layer strings.

    This can be achieved by providing a `custom_keys` function,
     which is a keys extension. i.e. returns a record of keys,
     extending a given `K` keys record.

    For example:

    * a keymap.ncl:
      """
      {
        custom_keys = fun K =>
          {
            MY_Q = K.Q & K.LeftShift
          },
        layers = [
          m%"
            MY_Q W E R T Y
          "%,
        ],
      }
      """
