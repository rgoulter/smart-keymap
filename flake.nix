{
  description = "smart-keymap — Nix flake for firmware-core OCI images";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    rgoulter-ch32 = {
      url = "github:rgoulter/ch32";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ./containers/flake-module.nix ];
      systems = [ "x86_64-linux" ];
    };
}
