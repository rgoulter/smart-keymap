{
  Check
    | doc "a Check is either a Bool, or a record with 'expected' and 'actual' fields."
    = std.contract.any_of [
      Bool,
      { expected | Dyn, actual | Dyn, message | String | optional },
    ],

  is_check
    | doc "predicate for if the given value is a Check."
    = fun c =>
      std.is_bool c ||
      std.is_record c && std.record.has_field "expected" c && std.record.has_field "actual" c,

  evaluate_check
    = fun c id =>
      if std.is_bool c then
        c == true || std.fail_with "%{id}: check failed"
      else
        (c.expected == c.actual) ||
          (let ser = fun v =>
            v |> std.typeof |> match {
              'Enum => v |> std.enum.to_tag_and_arg |> std.serialize 'Json,
              _ => v |> std.serialize 'Json,
            } in
            std.fail_with "%{id}: expected != actual:\n\nexpected:\n%{ser c.expected}\n\nactual:\n%{ser c.actual}"),

  NamedChecks
    | doc "a record, with Check values. e.g. { check_add = { expected = 4, actual = add 2 2 } }"
    = { _ | Check },

  CheckOrChecks
    = std.contract.any_of [
      Check,
      Checks
    ],

  Checks
    | doc "a record, with Check values"
    = { _ | CheckOrChecks },

  evaluate_check_or_checks
    = fun c id =>
     if is_check c then
        evaluate_check c id
      else
        std.record.map (fun k v => evaluate_check_or_checks v "%{id}.%{k}") c,

  evaluate_checks
    = fun c =>
        std.record.map (fun k v => evaluate_check_or_checks v k) c,

  checks | Checks | default = {},

  evaluated_checks
    = evaluate_checks checks,
}
