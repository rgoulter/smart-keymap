{
  validators,

  keymap_ncl.sequence
    | doc "for key::sequence::Key."
    = {
      Config = {
        required_idle_time | optional | Number,
        timeout | optional | Number,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { sequences = sequences_, passthrough = passthrough_key } =>
            validators.all_ok [
              sequences_
              |> validators.array.validator (
                validators.tuple.validator [validators.is_number, keymap_ncl.key.key_validator]
              ),
              keymap_ncl.key.key_validator passthrough_key,
            ],
          _ => 'Error { message = "expected { sequences = [[index, Key]], passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { sequences = sequences_, passthrough = passthrough_key } =>
        {
          sequences = sequences_ |> std.array.map (fun [i, sequence_key] => [i, keymap_ncl.key.to_json_value sequence_key]),
          passthrough = keymap_ncl.key.to_json_value passthrough_key,
        },

      map_accum = fun f acc _key @ { sequences, passthrough } =>
        let { acc, sequences } =
          sequences
          |> std.array.fold_left
            (fun { acc, sequences } [i, child] =>
              let result = f acc child in
              let sequences = sequences @ [[i, result.k]] in
              { acc = result.acc, include sequences }
            )
            { include acc, sequences = [] }
        in
        let result = f acc passthrough in
        {
          acc = result.acc,
          k = { include sequences, passthrough = result.k },
        },
    },

  keymap_ncl.sequence_aux
    | doc "for key::sequence::AuxiliaryKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { sequence_passthrough = passthrough_key } => keymap_ncl.key.key_validator passthrough_key,
          _ => 'Error { message = "expected { sequence_passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { sequence_passthrough = passthrough_key } =>
        {
          sequence_passthrough = keymap_ncl.key.to_json_value passthrough_key,
        },

      map_accum = fun f acc _key @ { sequence_passthrough } =>
        let result = f acc sequence_passthrough in
        {
          acc = result.acc,
          k = { sequence_passthrough = result.k },
        },
    },

  keymap_ncl.sequence_start
    | doc "for key::sequence::Ref::SequenceStart."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { sequence = "SequenceStart" } => 'Ok,
          _ => 'Error { message = "Expected { sequence = \"SequenceStart\" }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key =>
        key
        |> match {
          { sequence = json } => json,
          _ => std.fail_with "Expected { sequence = \"SequenceStart\" }",
        },

      map_accum = fun f acc k => { include acc, include k },
    },

  checks.check_sequence_start =
    let K = import "keys.ncl" in
    {
      check_sequence_start_ok =
        keymap_ncl.sequence_start.key_validator K.sequence_start == 'Ok,

      check_sequence_start_json = {
        actual = keymap_ncl.sequence_start.to_json_value K.sequence_start,
        expected = "SequenceStart",
      },

      # Record form merges with layered authoring as a layered base.
      check_sequence_start_layered_base_ok =
        keymap_ncl.key.key_validator (K.sequence_start & { layered = [K.A] }) == 'Ok,
    },
}
