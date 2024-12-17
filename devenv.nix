{ pkgs, lib, config, inputs, ... }:

{
  languages = {
    c.enable = true;
    ruby.enable = true;
    rust.enable = true;
  };

  packages = [
    pkgs.rust-cbindgen
  ];
}
