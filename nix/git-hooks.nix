{ pkgs, ... }:

let
  nickelFormat = pkgs.writeShellScriptBin "nickel-format" ''
    export PATH="${pkgs.lib.makeBinPath [ pkgs.nickel ]}:$PATH"
    exec ${./../ncl/scripts/ncl-format.sh} "$@"
  '';

  cargoCheckFirmware = pkgs.writeShellScriptBin "cargo-check-firmware" ''
    set -euo pipefail
    # Match firmware CI: no_std cross-compilation for smart-keymap.
    cargo check --no-default-features --target thumbv6m-none-eabi -p smart-keymap
    cargo check --no-default-features --target thumbv7em-none-eabihf -p smart-keymap
  '';
in
{
  git-hooks.hooks = {
    clippy = {
      enable = true;
      # Match .github/workflows/rust.yaml cargo-fmt-clippy job.
      entry = "cargo clippy --workspace --all-targets -- -D warnings";
      files = "\\.rs$";
      pass_filenames = false;
    };
    rustfmt.enable = true;

    cargo-check-firmware = {
      enable = true;
      name = "cargo check (no_std firmware targets)";
      entry = "${cargoCheckFirmware}/bin/cargo-check-firmware";
      language = "system";
      files = "\\.rs$";
      pass_filenames = false;
    };

    clang-format = {
      enable = true;
      files = "^((tests/ceedling/test)|(firmware/ch32x035-usb-device-compositekm-c/(app|keyboard))|(firmware/ch58x-ble-hid-keyboard-c/(APP|Profile|keyboard)))/.*\\.(c|h)$";
    };

    nickel-format = {
      enable = true;
      name = "nickel format";
      entry = "${nickelFormat}/bin/nickel-format";
      language = "system";
      files = "\\.ncl$";
      pass_filenames = true;
    };
  };
}
