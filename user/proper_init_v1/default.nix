{
  inputs,
  ...
}: let
  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
  rustToolchain = inputs.estros-rustc.packages.x86_64-linux.default;
  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

  # Whole workspace, filtered to cargo-relevant files. Vendoring + building all
  # four crates needs entry/uart/vfs/vfs_proto present together (vfs has a path
  # dep on ../vfs_proto).
  src = pkgs.lib.cleanSourceWith {
    src = ./.;
    filter = path: type:
      craneLib.filterCargoSources path type;
  };

  vendoredDeps = craneLib.vendorCargoDeps {
    inherit src;
  };

  mkRustCrate = {
    name,
    pkgName ? name,
    binName ? pkgName,
    buildType,
    driverBins ? {},
    output_name ? binName
  }: let
    profile =
      if buildType == "release"
      then "release"
      else "debug";
    driverEnv =
      pkgs.lib.concatMapStringsSep "\n"
      (name: "export ${pkgs.lib.toUpper name}_ELF=${driverBins.${name}}/${name}.elf")
      (pkgs.lib.attrNames driverBins);
  in
    pkgs.stdenv.mkDerivation {
      inherit name;
      inherit src;
      nativeBuildInputs = [
        rustToolchain
      ];

      buildPhase = ''
        export HOME=$TMPDIR
        mkdir -p .cargo
        cp ${vendoredDeps}/config.toml .cargo/config.toml
        ${driverEnv}
        cargo build --target aarch64-estros --offline -p ${pkgName} ${
          if profile == "release"
          then "--release"
          else ""
        }
        cp target/aarch64-estros/${profile}/${binName} ${output_name}.elf
      '';

      installPhase = ''
        mkdir $out
        cp ${output_name}.elf $out
      '';
    };

  # Driver binaries embedded into the init. Each entry maps a driver name to
  # its .elf build; the entry crate picks them up through
  # include_bytes!(env!("${upper}_ELF")) at compile time.
  mkDrivers = buildType: let
    mk = name: buildType:
      (mkRustCrate {
        inherit name buildType;
      })
      .outPath;
  in {
    uart = mk "uart" buildType;
    uart_user = mk "uart_user" buildType;
    vfs = mk "vfs" buildType;
  };

  mkRustInit = {name, buildType}:
    mkRustCrate {
      inherit name buildType;
    pkgName = "entry";
    binName = "entry";
    output_name = "init";
      driverBins = mkDrivers buildType;
    };

  # env-var name for a driver: driver name `uart` -> UART_ELF, consumed by
  # include_bytes!(env!("UART_ELF")) in entry/src/main.rs
in {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.rust_init = {
      release = {
        name = "rust_init";
        pkg = mkRustInit {
          name = "rust_init";
          buildType = "release";
        };
      };
      debug = {
        name = "rust_init";
        pkg = mkRustInit {
          name = "rust_init";
          buildType = "debug";
        };
      };
    };
  };
}
