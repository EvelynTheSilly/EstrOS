# convenience functions for building userspace stuff
{inputs,self, ...}:
let 
  singleFileInitBuild = 
      name:
      buildType:
      src:
      initName:
      let
        optFlag = if buildType == "debug" then "-O0" else "-O2";
        pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
      in
      pkgs.stdenv.mkDerivation {
        inherit name;
        src = src;
        nativeBuildInputs = [
          self.packages.x86_64-linux.aarch64-estros-binutils
        ];
        buildPhase = ''
          aarch64-estros-gcc ${optFlag} ${src}/${initName} -o init.elf
        '';
        installPhase = ''
          mkdir $out
          cp init.elf $out
        '';
      };
in
{
  flake =
      { ... }:
      {
        lib = {
          inherit singleFileInitBuild;
        };
      };
}
