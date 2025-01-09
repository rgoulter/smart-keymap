{ pkgs, lib, config, inputs, ... }:

{
  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };

  languages = {
    rust = {
      channel = "stable";
      enable = true;
      targets = ["thumbv6m-none-eabi"];
    };
    shell.enable = true;
  };

  packages = [
    pkgs.cargo-binutils
    pkgs.elf2uf2-rs
    pkgs.just
    pkgs.nickel
    pkgs.rust-cbindgen
    pkgs.yaml-language-server
  ];
}
