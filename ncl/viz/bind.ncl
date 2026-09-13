# Zip physical points with keymap keys. Default join is array order.
{
  identity_remap = fun n => std.array.generate (fun i => i) n,

  # remap[i] is the keymap index for points[i]. Empty remap → identity.
  zip = fun args =>
    let points = args.points in
    let keys = args.keys in
    let remap =
      if std.record.has_field "remap" args then
        args.remap
      else
        []
    in
    let n = std.array.length points in
    let remap =
      if remap == [] then
        identity_remap n
      else
        remap
    in
    let n_remap = std.array.length remap in
    let n_keys = std.array.length keys in
    if n != n_remap then
      std.fail_with "bind.zip: points length %{n} != remap length %{n_remap}"
    else
      points
      |> std.array.map_with_index (fun i p =>
        let ki = std.array.at i remap in
        if ki < 0 || ki >= n_keys then
          std.fail_with "bind.zip: remap[%{i}]=%{ki} out of keys[0..%{n_keys}]"
        else
          { point = p, key = std.array.at ki keys }
      ),

  checks = {
    check_identity =
      let points = [{ x = 0, y = 0, w = 1, h = 1, rot = 0 }, { x = 1, y = 0, w = 1, h = 1, rot = 0 }] in
      let keys = ["a", "b"] in
      let z = zip { include points, include keys } in
      {
        actual = z |> std.array.map (fun c => c.key),
        expected = ["a", "b"],
      },

    check_remap =
      let points = [{ x = 0, y = 0, w = 1, h = 1, rot = 0 }, { x = 1, y = 0, w = 1, h = 1, rot = 0 }] in
      let keys = ["a", "b"] in
      let z = zip { include points, include keys, remap = [1, 0] } in
      {
        actual = z |> std.array.map (fun c => c.key),
        expected = ["b", "a"],
      },
  },
}
