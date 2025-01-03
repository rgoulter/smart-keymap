{ pkgs, lib, config, inputs, ... }:

{
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
      targets = ["riscv32imac-unknown-none-elf"];
    };
  };

  packages = [
    pkgs.just
    pkgs.rust-cbindgen
    pkgs.yaml-language-server
  ];
}
