(import "smart-keys.ncl").key_extensions
& {
  extend_keys = fun K key_extension =>
    std.typeof key_extension
    |> match {
      'Record => K & key_extension,
      'Function => K & key_extension K,
    },
}
