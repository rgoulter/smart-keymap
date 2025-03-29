{ pkgs, lib, config, inputs, ... }:

let ch32 = inputs.rgoulter-ch32.packages.${pkgs.stdenv.system}; in
{
  devcontainer = {
    enable = true;
    settings.updateContentCommand = "";
  };

  languages = {
    c.enable = true;
    shell.enable = true;
  };

  packages = [
    pkgs.clang-tools
    pkgs.cmake
    pkgs.cmake-language-server
    pkgs.just
    pkgs.lldb
    pkgs.nickel

    ch32.wchisp
    ch32.xpack-riscv-none-elf-gcc
  ];
}
