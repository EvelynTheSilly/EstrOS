{inputs, ...}: let
  ccPackage = import ../../lib/c;

  mkInit = pkgs: buildType: file: let
    cc = ccPackage {inherit pkgs;};
    optFlag =
      if buildType == "debug"
      then "-O0"
      else "-O2";
  in
    pkgs.stdenv.mkDerivation {
      name = "send_and_receive";
      src = ./.;
      nativeBuildInputs = [
        cc.packages.aarch64-estros-binutils
      ];
      buildPhase = ''
        aarch64-estros-gcc ${optFlag} ${file} -o init.elf
      '';
      installPhase = ''
        mkdir $out
        cp init.elf $out
      '';
    };

  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
in {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.send = {
      release = {
        name = "send";
        pkg = mkInit pkgs "release" ./send.c;
      };
      debug = {
        name = "send";
        pkg = mkInit pkgs "debug" ./send.c;
      };
    };
    estros.inits.receive = {
      release = {
        name = "receive";
        pkg = mkInit pkgs "release" ./receive.c;
      };
      debug = {
        name = "receive";
        pkg = mkInit pkgs "debug" ./receive.c;
      };
    };
  };
}
