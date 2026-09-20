{
  key_extensions.key_lock = {
    # Record (not a bare string) so the key can merge with
    #  `{ layered = … }` / `{ named_layers = … }` as a layered base.
    key_lock = { key_lock = "KeyLock" },
  },
}
