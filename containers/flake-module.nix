{ inputs, ... }:
{
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      xpack = inputs'.rgoulter-ch32.packages.xpack-riscv-none-elf-gcc;

      rustToolchain = with inputs'.fenix.packages; combine [
        stable.rustc
        stable.cargo
        stable.clippy
        stable.rustfmt
        stable.rust-std
        targets.riscv32imac-unknown-none-elf.stable.rust-std
      ];

      pkgsFirmwareCore = [
        pkgs.bashInteractive
        pkgs.coreutils
        pkgs.findutils
        pkgs.gnused
        pkgs.gnugrep
        pkgs.git
        pkgs.just
        pkgs.gnumake
        pkgs.cmake
        pkgs.nickel
        pkgs.rust-cbindgen
        rustToolchain
        xpack
      ];

      pkgsFirmwareCoreCeedling = pkgsFirmwareCore ++ [
        pkgs.gcc
        pkgs.binutils
        pkgs.clang-tools
        pkgs.ruby_3_3
        pkgs.ceedling
      ];

      mkEnv = name: paths: pkgs.buildEnv {
        inherit name;
        inherit paths;
        pathsToLink = [ "/bin" ];
        extraOutputsToInstall = [ "out" ];
      };

      workspaceRoot = pkgs.runCommand "workspace-root" { } "mkdir -p $out/workspace";

      mkImage = name: tag: env: pkgs.dockerTools.buildLayeredImage {
        inherit name;
        inherit tag;
        contents = [ env workspaceRoot ];
        config = {
          Cmd = [ "/bin/bash" ];
          WorkingDir = "/workspace";
          Env = [
            "PATH=/bin:${env}/bin"
            "RUSTUP_TOOLCHAIN=none"
          ];
        };
        maxLayers = 120;
      };

      envFirmwareCore = mkEnv "smart-keymap-firmware-core" pkgsFirmwareCore;
      envFirmwareCoreCeedling = mkEnv "smart-keymap-firmware-core-ceedling" pkgsFirmwareCoreCeedling;
    in
    {
      packages = {
        env-firmware-core = envFirmwareCore;
        env-firmware-core-ceedling = envFirmwareCoreCeedling;
        image-firmware-core = mkImage "smart-keymap-firmware-core" "latest" envFirmwareCore;
        image-firmware-core-ceedling = mkImage "smart-keymap-firmware-core-ceedling" "latest" envFirmwareCoreCeedling;
        xpack-riscv-none-elf-gcc = xpack;
        rust-toolchain = rustToolchain;
        default = self'.packages.image-firmware-core;
      };
    };
}
