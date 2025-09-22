{
  key_extensions = (import "key-extensions.ncl") |> std.record.remove "extend_keys",

  grouped_key_fields =
    std.record.map_values
      (fun key_extension =>
        key_extension
        |> std.record.fields
        |> std.array.filter ((!=) "keys")
      )
      key_extensions,

  keys_md = m%"
    # `keys.ncl` Fields

    `keys.ncl` provides the values for key definitions
     used in `keymap.ncl`.

    See the documentation for [keymap.ncl](#keymap.ncl)
     for examples of using `keys.ncl`.

    The fields available in `keys.ncl`:

    %{
      grouped_key_fields
      |> std.record.map
           (fun extension_group_name key_extension_fields =>
             m%"
                ### %{extension_group_name}
                %{
                  key_extension_fields
                  |> std.array.map (fun field_name => "- `` %{field_name} ``")
                  |> std.string.join "\n"
                }
                "%)
      |> std.record.values
      |> std.string.join "\n\n"
    }
    "%,
}
