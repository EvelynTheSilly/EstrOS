{ inputs, lib, ... }:
let
  kernelSrc = ../../kernel;
in
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    {
      inputs',
      self',
      pkgs,
      ...
    }:
    let
      estros-lib = import ./_helpers.nix {
        nixpkgs = inputs.nixpkgs;
        rust-overlay = inputs.rust-overlay;
      };

      rust = self'.packages.rust;
      cross = pkgs.pkgsCross.aarch64-embedded;

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      sysrootLock = "${rust}/lib/rustlib/src/rust/library/Cargo.lock";
      kernelDeps = rustPlatform.importCargoLock { lockFile = "${kernelSrc}/Cargo.lock"; };
      sysrootDeps = rustPlatform.importCargoLock { lockFile = sysrootLock; };
      combinedDeps = pkgs.runCommand "cargo-vendor-dir" { } ''
        mkdir -p $out
        cp -rL ${sysrootDeps}/* $out/
        chmod -R u+w $out
        cp -rL ${kernelDeps}/* $out/
      '';
    in
    {
      packages.kernel_elf = rustPlatform.buildRustPackage {
        name = "estros_kernel";
        src = kernelSrc;

        cargoDeps = combinedDeps;

        buildType = "debug";
        doCheck = false;
        auditable = false;

        env.INIT_ELF_PATH = "${self'.packages.init}/init.elf";

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
          cp ./target/aarch64-none-custom/debug/kernel $out/kernel.elf
          runHook postInstall
        '';
      };
    };
}
