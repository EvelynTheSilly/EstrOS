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
      kernel = self'.packages.kernel_elf;
      limine = pkgs.limine-full;
      ovmf = pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd;

      build = import ./_build.nix { inherit pkgs kernel limine ovmf; };
      mkScript = import ./_scripts.nix;

      run =
        mkScript {
          inherit pkgs;
          name = "estros-run";
          inherit (build) efiVars diskImage;
        };
      debug =
        mkScript {
          inherit pkgs;
          name = "estros-debug";
          inherit (build) efiVars diskImage;
          extraFlags = "-S -s";
        };

      init = self'.packages.init;
      cross = pkgs.pkgsCross.aarch64-embedded;
      isLinux = system != "aarch64-darwin";
    in
    {
      packages = {
        inherit run debug;
        default = run;

        krun = pkgs.writeShellScriptBin "krun" ''
          exec ${run}/bin/estros-run "$@"
        '';
        kdebug = pkgs.writeShellScriptBin "kdebug" ''
          exec ${debug}/bin/estros-debug "$@"
        '';
      } // pkgs.lib.optionalAttrs isLinux {
        gdb = pkgs.writeShellScriptBin "gdb" ''
          tmpgdbinit=$(mktemp)
          trap 'rm -f "$tmpgdbinit"' EXIT
          sed \
            -e 's|KERNEL_ELF_PATH|${kernel}/kernel.elf|g' \
            -e 's|INIT_ELF_PATH|${init}/init.elf|g' \
            ${./gdbinit} > "$tmpgdbinit"
          exec ${cross.buildPackages.gdb}/bin/aarch64-none-elf-gdb -ix "$tmpgdbinit" "$@"
        '';
      };
    };
}
