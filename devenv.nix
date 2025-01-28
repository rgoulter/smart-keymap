{ pkgs, lib, config, inputs, ... }:

{
  devcontainer = {
    enable = true;
    settings.updateContentCommand = "";
  };

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };

  languages = {
    c.enable = true;
    ruby.enable = true;
    rust = {
      channel = "stable";
      enable = true;
      targets = ["riscv32imac-unknown-none-elf" "thumbv6m-none-eabi"];
    };
    shell.enable = true;
  };

  packages = [
    pkgs.cmake-language-server
    pkgs.cargo-binutils
    pkgs.cargo-nextest
    pkgs.elf2uf2-rs
    pkgs.just
    pkgs.nickel
    pkgs.rust-cbindgen
    pkgs.yaml-language-server
  ];
}
