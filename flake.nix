{
  description = "estros devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        cross = pkgs.pkgsCross.aarch64-embedded;
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnsupportedSystem = true;
        };
        toolchainToml = fromTOML (builtins.readFile ./rust-toolchain.toml);

        toolchain = toolchainToml.toolchain;

        rust = pkgs.rust-bin.fromRustupToolchain {
          channel = toolchain.channel;
          components = toolchain.components or [ ];
          targets = toolchain.targets or [ ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust
            bacon
            pkg-config
            openssl
            cross.buildPackages.gcc
            cross.buildPackages.binutils
            (if system != "aarch64-darwin" then cross.buildPackages.gdb else null)
            qemu
            cmake
            just
            just-lsp
            just-formatter
            cloc
            mtools
            pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd
            OVMF.fd
            gptfdisk
            python313Packages.virt-firmware
          ];
          LIMINE_EFI_PATH = "${pkgs.limine-full}/share/limine/BOOTAA64.EFI";
          BOOT_FIRMWARE_PATH = "${pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd}/FV";
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
