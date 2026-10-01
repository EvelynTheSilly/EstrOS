{ inputs, ... }:
let
  ccPackage = import ../../lib/c;

  mkInit =
    pkgs:
    buildType:
    let
      cc = ccPackage { inherit pkgs; };
      optFlag = if buildType == "debug" then "-O0" else "-O2";
    in
    pkgs.stdenv.mkDerivation {
      name = "process_spawner";
      src = ./.;
      nativeBuildInputs = [
        cc.packages.aarch64-estros-binutils
        pkgs.xxd
      ];
      buildPhase = ''
        aarch64-estros-gcc ${optFlag} second.c -o second.elf
        xxd -i -n second_process_elf second.elf > second_process_elf.h
        aarch64-estros-gcc ${optFlag} main.c -o init.elf
      '';
      installPhase = ''
        mkdir $out
        cp init.elf $out
      '';
    };

  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
in
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.process_spawner = {
        release = {
          name = "process_spawner";
          pkg = mkInit pkgs "release";
        };
        debug = {
          name = "process_spawner";
          pkg = mkInit pkgs "debug";
        };
      };
    };
}
