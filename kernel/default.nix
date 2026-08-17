{
  pkgs,
  rust,
  cross,
  opt,
  init,
  ...
}:
let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
  sysrootLock = "${rust}/lib/rustlib/src/rust/library/Cargo.lock";
  kernelDeps = rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
  sysrootDeps = rustPlatform.importCargoLock { lockFile = sysrootLock; };
  combinedDeps = pkgs.runCommand "cargo-vendor-dir" { } ''
    mkdir -p $out
    cp -rL ${sysrootDeps}/* $out/
    chmod -R u+w $out
    cp -rL ${kernelDeps}/* $out/
  '';
in
rustPlatform.buildRustPackage {
  name = "estros_kernel";
  src = ./.;

  cargoDeps = combinedDeps;

  buildType = opt.level;
  doCheck = false;
  auditable = false;

  env.INIT_ELF_PATH = "${init}/init.elf";

  nativeBuildInputs = [
    cross.buildPackages.gcc
  ];

  buildPhase = ''
    runHook preBuild
    ${rust}/bin/cargo build --bin kernel -Z json-target-spec --locked --offline --target aarch64-none-custom.json
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir $out
    cp ./target/aarch64-none-custom/${opt.level}/kernel $out/kernel.elf
    runHook postInstall
  '';
}
