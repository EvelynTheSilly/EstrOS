{
  description = "estros devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        cross = pkgs.pkgsCross.aarch64-embedded;
        overlays = [rust-overlay.overlays.default];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnsupportedSystem = true;
        };
        toolchainToml = fromTOML (builtins.readFile ./rust-toolchain.toml);

        toolchain = toolchainToml.toolchain;

        rust = pkgs.rust-bin.fromRustupToolchain {
          channel = toolchain.channel;
          components = toolchain.components or [];
          targets = toolchain.targets or [];
        };

        aarch64-estros-binutils = pkgs.wrapCCWith {
          cc = cross.buildPackages.gcc;
          bintools = cross.buildPackages.binutils;
          extraTools = with cross.buildPackages; [
            binutils
            binutils-unwrapped
            elfutils
          ];
        };
      in {
        packages.aarch64-estros-binutils = aarch64-estros-binutils;

        devShells.default = pkgs.mkShell {
          packages =
            [
              pkgs.cargo-expand
              rust
              pkgs.bacon
              pkgs.pkg-config
              pkgs.openssl
              pkgs.qemu
              pkgs.cmake
              pkgs.just
              pkgs.just-lsp
              pkgs.just-formatter
              pkgs.cloc
            ]
            ++ (
              if system != "aarch64-darwin"
              then [
                aarch64-estros-binutils
                cross.buildPackages.gdb
                pkgs.mtools
                pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd
                pkgs.OVMF.fd
                pkgs.gptfdisk
                pkgs.python313Packages.virt-firmware
              ]
              else []
            );
          LIMINE_EFI_PATH = (if system != "aarch64-darwin" then "${pkgs.limine-full}/share/limine/BOOTAA64.EFI" else "");
          BOOT_FIRMWARE_PATH = (if system != "aarch64-darwin" then "${pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd}/FV" else "");
          CARGO_UNSTABLE_JSON_TARGET_SPEC = "true";
          shellHook = ''
            if [[ $- == *i* ]]; then
                nu -e "alias cloc = cloc --vcs git; alias bacon = bacon -- -Z json-target-spec; alias cargo = cargo -Z json-target-spec "
            fi
          '';
        };
      }
    );
}
