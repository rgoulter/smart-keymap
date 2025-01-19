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
    },

  checks.check_record_validators = {
    check_has_any_field_of
      = 'Ok == record.has_any_field_of ["x", "y"] { x = 3, z = 5 },

    check_has_only_fields
      = 'Ok == record.has_only_fields ["x", "y"] { x = 3 },
  },

  record = {
    has_any_field_of
      = fun fields r =>
        if r |> std.record.fields |> std.array.length == 0 then
          'Error { message = "record has no fields" }
        else if r |> std.record.fields |> std.array.any (fun f => std.array.elem f fields) then
          'Ok
        else
          'Error {
            message = "Unexpected fields",
            notes = [
              "Expected any of: %{fields |> std.serialize},",
              "Found: %{r |> std.record.fields}"
            ],
          },

    has_only_fields
      = fun fields r =>
        if r |> std.record.fields |> std.array.all (fun f => std.array.elem f fields) then
          'Ok
        else
          'Error {
            message = "Unexpected fields",
            notes = [
              "Expected only: %{fields |> std.serialize},",
              "Found: %{r |> std.record.fields}"
            ],
          },
  },
}
