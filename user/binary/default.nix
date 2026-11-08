{inputs, ...}: let
  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;

  pkg = pkgs.runCommand "binary" {} ''
    mkdir -p $out
    cp ${./init.elf} $out/init.elf
  '';
in {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.binary = {
      release = {
        name = "binary";
        inherit pkg;
      };
      debug = {
        name = "binary";
        inherit pkg;
      };
    };
  };
}
