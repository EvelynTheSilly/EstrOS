{
  pkgs,
  rust,
  cross,
  opt,
  ...
}:
pkgs.stdenv.mkDerivation {
  name = "estros_kernel";
  src = ./.;

  nativeBuildInputs = [
    cross.buildPackages.gcc
    rust
  ];
  buildPhase = ''
    ${rust}/bin/cargo build ${if opt.level == "debug" then "" else "--release"} -Z json-target-spec
  '';
  installPhase = ''
    mkdir $out
    cp ./target/aarch64-none-custom/${opt.level}/kernel $out/kernel.elf
  '';
}
