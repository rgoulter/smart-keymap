{
  prefix = "KM_",

  keymap = import "keymap.ncl",

  fixture = keymap.ceedling_fixture,

  suite = fixture.suite,

  fixture_entries =
    fixture
    |> std.record.to_array
    |> std.array.filter (fun { field, .. } => field != "suite"),

  ceedling_fixture_h =
    let header =
      "#pragma once\n\n"
      ++ "#define SUITE_%{suite} 1\n"
    in
    let defines =
      fixture_entries
      |> std.array.map (fun { field, value } => "#define %{prefix}%{field} %{std.to_string value}")
      |> std.string.join "\n"
    in
    if defines == "" then
      header
    else
      header ++ "\n" ++ defines,
}
