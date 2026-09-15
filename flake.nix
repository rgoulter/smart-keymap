{
  description = "smart-keymap — Nix flake for firmware-core OCI images";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/e8be7818e19ada32105a8af937a6a473b38167ca";
    fenix = {
      url = "github:nix-community/fenix/4bb440a5be00604edd7a663f6813346f3901b5c1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    rgoulter-ch32 = {
      url = "github:rgoulter/ch32/1d0895de72ee7674e167b520bf2798f53fe8a1da";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ./containers/flake-module.nix ];
      systems = [ "x86_64-linux" ];
    };
}
