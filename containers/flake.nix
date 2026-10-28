{
  description = "Pre-baked OCI for smart-keymap CH32 firmware (+ optional Ceedling) — re-exports root flake";

  inputs = {
    root.url = "path:..";
  };

  outputs = { self, root }: {
    packages.x86_64-linux = root.packages.x86_64-linux;
  };
}
