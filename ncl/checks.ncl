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
    = fun c =>
      if std.is_bool c then
        c == true || std.fail_with "check failed"
      else
        (c.expected == c.actual) ||
          std.fail_with "expected equal: %{std.serialize 'Json c}",

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
    = fun c =>
     if is_check c then
        evaluate_check c
      else
        std.record.map_values evaluate_check_or_checks c,

  evaluate_checks
    = fun c => std.record.map_values evaluate_check_or_checks c,

  checks | Checks | default = {},

  evaluated_checks
    = evaluate_checks checks,
}
