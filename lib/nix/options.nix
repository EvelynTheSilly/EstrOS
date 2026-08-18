{ lib, ... }:
{
  options.estros = {
    init = lib.mkOption {
      type = lib.types.package;
      description = "The cross-compiled init ELF to embed in the kernel";
    };
    system = lib.mkOption {
      type = lib.types.str;
      default = builtins.currentSystem;
      description = "The host system for the build toolchain";
    };
  };

  config.systems = [
    "x86_64-linux"
    "aarch64-linux"
    "aarch64-darwin"
  ];
}
