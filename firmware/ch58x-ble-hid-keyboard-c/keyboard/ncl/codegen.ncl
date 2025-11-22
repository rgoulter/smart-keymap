{
  separator | default = " ",

  cmakelists | default = {},
  cmakelists_filenames = cmakelists |> std.record.fields |> std.string.join separator,

  includes | default = {},
  includes_filenames = includes |> std.record.fields |> std.string.join separator,

  sources | default = {},
  source_filenames = sources |> std.record.fields |> std.string.join separator,
}
