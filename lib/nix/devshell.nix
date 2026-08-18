{ inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    {
      inputs',
      self',
      pkgs,
      system,
      ...
    }:
    let
      cc = (import ../c { inherit pkgs; });
      cross = pkgs.pkgsCross.aarch64-embedded;
      rust = self'.packages.rust;
      isLinux = system != "aarch64-darwin";
    in
    {
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
            self'.packages.krun
            self'.packages.kdebug
          ]
          ++ pkgs.lib.optionals isLinux [
            cross.buildPackages.gcc
            cc.packages.aarch64-estros-binutils
            self'.packages.gdb
            pkgs.mtools
            pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd
            pkgs.OVMF.fd
            pkgs.gptfdisk
            pkgs.python313Packages.virt-firmware
          ];
        LIMINE_EFI_PATH = if isLinux then "${pkgs.limine-full}/share/limine/BOOTAA64.EFI" else "";
        BOOT_FIRMWARE_PATH = if isLinux then "${pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd}/FV" else "";
        CARGO_UNSTABLE_JSON_TARGET_SPEC = "true";
        shellHook = ''
          if [[ $- == *i* ]]; then
              nu -e "alias cloc = cloc --vcs git; alias bacon = bacon -- -Z json-target-spec; alias cargo = cargo -Z json-target-spec "
          fi
        '';
      };
    };
}
