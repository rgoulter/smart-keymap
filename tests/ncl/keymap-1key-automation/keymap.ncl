let K = import "keys.ncl" in

let MY_MACRO = K.string_macro "a" in
{
  keys = [
    MY_MACRO,
  ],
}
