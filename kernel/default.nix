{
  craneLib,
  pkgs,
  rust,
  buildType ? "release",
}: let
  cross = pkgs.pkgsCross.aarch64-embedded;
  isRelease = buildType == "release";

  src = pkgs.lib.cleanSourceWith {
    src = ./.;
    filter = path: type: let
      name = builtins.baseNameOf (builtins.toString path);
    in
      craneLib.filterCargoSources path type
      || name == "aarch64-none-custom.json"
      || name == "linker.ld";
  };

  cargoProfileDir =
    if isRelease
    then "release"
    else "debug";
  profileArg =
    if isRelease
    then "--release"
    else "--profile dev";

  commonArgs = {
    inherit src;
    pname = "estros-kernel";
    version = "0.1.0";
    strictDeps = true;
    doCheck = false;

    cargoVendorDir = craneLib.vendorMultipleCargoDeps {
      inherit (craneLib.findCargoFiles src) cargoConfigs;
      cargoLockList = [
        "${./.}/Cargo.lock"
        "${rust}/lib/rustlib/src/rust/library/Cargo.lock"
      ];
    };

    cargoBuildCommand = "cargo build ${profileArg}";
    cargoCheckCommand = "cargo check ${profileArg}";
    cargoTestCommand = "cargo test ${profileArg}";
    cargoExtraArgs = "-Z json-target-spec --locked --target aarch64-none-custom.json --bin kernel";

    nativeBuildInputs = [
      cross.buildPackages.gcc
    ];

    extraDummyScript = ''
      cp ${./.}/aarch64-none-custom.json $out/aarch64-none-custom.json
    '';
  };

  cargoArtifacts = craneLib.buildDepsOnly (commonArgs
    // {
      pname = "estros-kernel-deps";
    });

  kernel = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;

      installPhaseCommand = ''
        mkdir -p $out
        cp target/aarch64-none-custom/${cargoProfileDir}/kernel $out/kernel.elf
      '';
    });
in
  kernel
