{
  description = "Pre-baked OCI for smart-keymap CH32 firmware (+ optional Ceedling)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/e8be7818e19ada32105a8af937a6a473b38167ca";
    fenix = {
      url = "github:nix-community/fenix/4bb440a5be00604edd7a663f6813346f3901b5c1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      xpack = pkgs.callPackage ../nix/xpack-riscv-none-elf-gcc { inherit system; };

      rustToolchain = with fenix.packages.${system}; combine [
        stable.rustc
        stable.cargo
        stable.rust-std
        targets.riscv32imac-unknown-none-elf.stable.rust-std
      ];

      pkgsFirmwareCore = [
        pkgs.bashInteractive pkgs.coreutils pkgs.findutils pkgs.gnused pkgs.gnugrep
        pkgs.just pkgs.gnumake pkgs.cmake
        pkgs.nickel pkgs.rust-cbindgen
        rustToolchain
        xpack
      ];

      pkgsFirmwareCoreCeedling = pkgsFirmwareCore ++ [
        pkgs.gcc pkgs.binutils
        pkgs.ruby_3_3 pkgs.ceedling
      ];

      mkEnv = name: paths: pkgs.buildEnv {
        inherit name;
        inherit paths;
        pathsToLink = [ "/bin" ];
        extraOutputsToInstall = [ "out" ];
      };

      mkImage = name: tag: env: pkgs.dockerTools.buildLayeredImage {
        inherit name tag;
        contents = [ env ];
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
    in {
      packages.${system} = {
        env-firmware-core = envFirmwareCore;
        env-firmware-core-ceedling = envFirmwareCoreCeedling;
        image-firmware-core = mkImage "smart-keymap-firmware-core" "latest" envFirmwareCore;
        image-firmware-core-ceedling = mkImage "smart-keymap-firmware-core-ceedling" "latest" envFirmwareCoreCeedling;
        xpack-riscv-none-elf-gcc = xpack;
        rust-toolchain = rustToolchain;
        default = mkImage "smart-keymap-firmware-core" "latest" envFirmwareCore;
      };
    };
}
