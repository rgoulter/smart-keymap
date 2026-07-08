{ pkgs, lib, config, inputs, ... }:

{
  imports = [ ./nix/git-hooks.nix ];

  devcontainer = {
    enable = true;
    settings.updateContentCommand = "sudo setfacl -k /tmp";
  };

  languages = {
    c.enable = true;
    rust = {
      channel = "stable";
      components = [ "rustc" "cargo" "clippy" "llvm-tools-preview" "rustfmt" "rust-analyzer" ];
      enable = true;
      targets = ["riscv32imac-unknown-none-elf" "thumbv6m-none-eabi" "thumbv7em-none-eabihf"];
    };
    shell.enable = true;
  };

  packages = let
    uf2conv = pkgs.callPackage ./nix/uf2conv {};
  in [
    pkgs.cargo-binutils
    pkgs.cargo-deny
    pkgs.cargo-edit
    pkgs.cargo-nextest
    pkgs.cargo-release
    pkgs.ceedling
    pkgs.clang-tools
    pkgs.cmake-language-server
    pkgs.elf2uf2-rs
    pkgs.gh
    pkgs.just
    pkgs.lldb
    pkgs.nickel
    pkgs.nls
    pkgs.picotool
    pkgs.rust-cbindgen
    pkgs.yaml-language-server
    uf2conv
  ];
}
