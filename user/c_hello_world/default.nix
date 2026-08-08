{
  pkgs,
  cross,
  cc,
  ...
}:
pkgs.stdenv.mkDerivation {
  name = "c_hello_world";
  src = ./.;
  buildPhase = ''
    ${cc}/bin/aarch64-estros-gcc main.c -o init.elf
  '';
  installPhase = ''
    mkdir $out
    cp init.elf $out
  '';
}
