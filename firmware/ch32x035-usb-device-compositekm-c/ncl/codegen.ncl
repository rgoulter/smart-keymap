{
  cmakelists | default = {},
  cmakelists_filenames = cmakelists |> std.record.fields |> std.string.join " ",

  includes | default = {},
  includes_filenames = includes |> std.record.fields |> std.string.join " ",

  sources | default = {},
  source_filenames = sources |> std.record.fields |> std.string.join " ",
}
