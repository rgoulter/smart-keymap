{
  sources,
  source_filenames = sources |> std.record.fields |> std.string.join " "
}
