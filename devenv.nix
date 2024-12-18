{ pkgs, lib, config, inputs, ... }:

{
  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };

  languages = {
    c.enable = true;
    ruby.enable = true;
    rust.enable = true;
  };

  packages = [
    pkgs.rust-cbindgen
  ];
}
