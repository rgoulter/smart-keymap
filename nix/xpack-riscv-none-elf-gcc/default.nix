# Vendored from github:rgoulter/ch32 (nix/xpack-riscv-none-elf-gcc) — correct
# `riscv-none-elf-*` prefix that firmware/toolchains/riscv-none-elf.cmake expects.
{ lib
, stdenv
, fetchurl
, system
, autoPatchelfHook
}:

let
  urls = {
    "aarch64-darwin" = {
      os = "darwin-arm64";
      hash = "sha256-526GuMUA+OkrO0/3sERM+/OyGFFfMikp4HROw7ntgKg=";
    };
    "x86_64-darwin" = {
      os = "darwin-x64";
      hash = "sha256-im5pnxKHYVLWOG53dnXZRSnMwhpXIkpp2XP2dpSaFoc=";
    };
    "x86_64-linux" = {
      os = "linux-x64";
      hash = "sha256-9XRBW2PxKwm900dSI6tJKkZdI4EGRskME6TDtnbINQM=";
    };
  };
  version = "14.2.0-3";
  url = "https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases/download/v${version}/xpack-riscv-none-elf-gcc-${version}-${urls."${system}".os}.tar.gz";
in
stdenv.mkDerivation {
  pname = "xpack-riscv-none-elf-gcc";
  inherit version;

  src = fetchurl {
    inherit url;
    hash = urls."${system}".hash;
  };

  dontConfigure = true;
  dontBuild = true;
  dontStrip = true;
  noAuditTmpdir = true;

  nativeBuildInputs = lib.lists.optional stdenv.hostPlatform.isLinux [ autoPatchelfHook ];

  autoPatchelfIgnoreMissingDeps = [
    "libcrypt.so.2" "libcrypto.so.3" "libexpat.so.1" "libffi.so.8" "libfl.so.2"
    "libgcc_s.so.1" "libgmp.so.10" "libiconv.so.2" "libisl.so.23" "liblzma.so.5"
    "libmpc.so.3" "libmpfr.so.6" "libncurses.so.6" "libnsl.so.1" "libpanel.so.6"
    "libpython3.12.so.1.0" "libreadline.so.8" "libsqlite3.so.0" "libssl.so.3"
    "libstdc++.so.6" "libz.so.1" "libzstd.so.1"
  ];

  installPhase = ''
    mkdir -p $out/
    cp -r . $out
  '';
}
