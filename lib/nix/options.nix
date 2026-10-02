{ lib, ... }:
{
  options.flake.estros.inits = lib.mkOption {
    type = lib.types.attrsOf (lib.types.attrsOf (lib.types.submodule {
      options = {
        name = lib.mkOption { type = lib.types.str; };
        pkg = lib.mkOption { type = lib.types.package; };
      };
    }));
    default = { };
    description = "Named init definitions grouped by source";
  };

  options.flake.lib = lib.mkOption {
    type = lib.types.attrsOf lib.types.anything;
    default = { };
    description = "Library functions";
  };

  config.systems = [
    "x86_64-linux"
    "aarch64-linux"
    "aarch64-darwin"
  ];
}
