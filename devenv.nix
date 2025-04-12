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
      components = [ "rustc" "cargo" "clippy" "llvm-tools-preview" "rustfmt" "rust-analyzer" ];
      enable = true;
      targets = ["riscv32imac-unknown-none-elf" "thumbv6m-none-eabi" "thumbv7em-none-eabihf"];
    };
    shell.enable = true;
  };

  packages = [
    pkgs.cargo-binutils
    pkgs.cargo-deny
    pkgs.cargo-nextest
    pkgs.clang-tools
    pkgs.cmake-language-server
    pkgs.elf2uf2-rs
    pkgs.just
    pkgs.lldb
    pkgs.nickel
    pkgs.nls
    pkgs.rust-cbindgen
    pkgs.yaml-language-server
  ];
}
