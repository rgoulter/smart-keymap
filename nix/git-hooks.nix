{ pkgs, ... }:

let
  nickelFormat = pkgs.writeShellScriptBin "nickel-format" ''
    export PATH="${pkgs.lib.makeBinPath [ pkgs.nickel ]}:$PATH"
    exec ${./../ncl/scripts/ncl-format.sh} "$@"
  '';
in
{
  git-hooks.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;

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
