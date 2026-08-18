{ inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    { pkgs, ... }:
    let
      cc = (import ../c { inherit pkgs; });
    in
    {
      packages.init = pkgs.stdenv.mkDerivation {
        name = "c_hello_world";
        src = ../../user/c_hello_world;
        nativeBuildInputs = [
          cc.packages.aarch64-estros-binutils
        ];
        buildPhase = ''
          aarch64-estros-gcc main.c -o init.elf
        '';
        installPhase = ''
          mkdir $out
          cp init.elf $out
        '';
      };
    };

  flake =
    { config, self, ... }:
    {
      estros.init = self.packages.${config.estros.system}.init;
    };
}
