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

        sysroot = ./lib/c/sysroot;

        estros-gcc = cross.buildPackages.wrapCCWith {
          cc = cross.buildPackages.gcc.cc;
          bintools = cross.buildPackages.binutils;
          extraTools = with cross.buildPackages; [
            binutils
            binutils-unwrapped
            elfutils
          ];
          extraBuildCommands = ''
            echo "-isystem ${sysroot}/include" >> $out/nix-support/cc-cflags
            echo "-L ${sysroot}/lib" >> $out/nix-support/cc-ldflags
            cat > $out/nix-support/estros.specs <<EOF
*startfile:
%{!shared: ${sysroot}/lib/crt0.o}
*lib:
%{!shared: -lestros}
EOF
            echo "-static -specs=$out/nix-support/estros.specs" >> $out/nix-support/cc-cflags
          '';
        };

        aarch64-estros-binutils = pkgs.runCommandLocal "aarch64-estros-binutils" {
          wrapped = estros-gcc;
        } ''
          mkdir -p $out/bin $out/nix-support
          for src in $wrapped/bin/*; do
            name=$(basename "$src")
            ln -s "$src" "$out/bin/''${name/aarch64-none-elf-/aarch64-estros-}"
          done
          cat > $out/nix-support/setup-hook <<EOF
          addToSearchPath _PATH $out/bin
          EOF
        '';
      in
      {
        packages.aarch64-estros-binutils = aarch64-estros-binutils;

        devShells.default = pkgs.mkShell {
          packages = [
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
            if system != "aarch64-darwin" then
              [
                cross.buildPackages.gcc
                aarch64-estros-binutils
                cross.buildPackages.gdb
                pkgs.mtools
                pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd
                pkgs.OVMF.fd
                pkgs.gptfdisk
                pkgs.python313Packages.virt-firmware
              ]
            else
              [ ]
          );
          LIMINE_EFI_PATH = (
            if system != "aarch64-darwin" then "${pkgs.limine-full}/share/limine/BOOTAA64.EFI" else ""
          );
          BOOT_FIRMWARE_PATH = (
            if system != "aarch64-darwin" then "${pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd}/FV" else ""
          );
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
