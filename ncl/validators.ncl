{
  Validator
    = Dyn ->
      [|
        'Ok,
        'Error { message | String | optional, notes | Array String | optional, }
      |],

  check_is_error
    = fun c =>
      c |> match {
        'Error _ => true,
        _ => false,
      },

  ok
    = std.function.const 'Ok,

  fail
    = std.function.const ('Error {}),

  is_array
    = fun v =>
      if std.is_array v then
        'Ok
      else
        'Error { message = "Expected array" },

  is_bool
    = fun v =>
      if std.is_bool v then
        'Ok
      else
        'Error { message = "Expected bool" },

  is_null
    = fun v =>
      if v == null then
        'Ok
      else
        'Error { message = "Expected null" },

  is_number
    = fun v =>
      if std.is_number v then
        'Ok
      else
        'Error { message = "Expected number" },

  is_record
    = fun v =>
      if std.is_record v then
        'Ok
      else
        'Error { message = "Expected record" },

  is_string
    = fun v =>
      if std.is_string v then
        'Ok
      else
        'Error { message = "Expected string" },

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

    check_validator
      = 'Ok
        ==
        record.validator
          {
            fields_validator = record.has_only_fields ["x", "y"],
            field_validators = {
              x = is_number,
            },
          }
          { x = 3 },
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

    validator
      = fun { fields_validator, field_validators } r =>
        r |> fields_validator |> match {
          'Ok =>
            let rec validate_fields = fun fields =>
              fields |> match {
                [] => 'Ok,
                [field, ..fields] =>
                  (if std.record.has_field field field_validators then
                     let v = std.record.get field r in
                     let validator = std.record.get field field_validators in
                     validator v
                   else
                     'Ok) |> match {
                    'Ok => validate_fields fields,
                    err => err,
                  }
              } in
              r |> std.record.fields |> validate_fields,

          err => err,
        },
  },
}
