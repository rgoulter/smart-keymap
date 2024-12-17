{ pkgs, lib, config, inputs, ... }:

{
  languages = {
    c.enable = true;
    rust.enable = true;
  };

  packages = [
    pkgs.git
    pkgs.meson
    pkgs.ninja
    pkgs.rust-cbindgen
  ];
}
