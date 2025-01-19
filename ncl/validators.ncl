{
  Validator
    = Dyn ->
      [|
        'Ok,
        'Error { message | String | optional, notes | Array String | optional, }
      |],

  ok
    = std.function.const 'Ok,

  fail
    = std.function.const ('Error {}),

  checks.all_of = {
    all_ok_is_ok = {
      expected = 'Ok,
      actual = all_of [ok, ok, ok] {},
    },
    any_fail_is_error = {
      expected = 'Error {},
      actual = all_of [ok, fail, ok] {},
    },
  },

  all_of = fun validators v =>
    validators |> match {
      [] => 'Ok,
      [validator, ..validators] =>
        validator v |> match {
          'Ok => all_of validators v,
          err => err,
        },
    }
}
