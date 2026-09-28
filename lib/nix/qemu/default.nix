{ inputs, self, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    {
      self',
      pkgs,
      system,
      ...
    }:
    let
      cross = pkgs.pkgsCross.aarch64-embedded;
      isLinux = system != "aarch64-darwin";
      inits = self.estros.inits;
      releaseInit = self.lib.buildInit { init = inits.userspace_printer.release; };
      debugInit = self.lib.buildInit { init = inits.userspace_printer.debug; };

      release = self.lib.qemu.buildDiskImage {
        init = releaseInit;
        kernel = self'.packages.kernel_elf;
        inherit pkgs;
      };
      debug = self.lib.qemu.buildDiskImage {
        init = debugInit;
        kernel = self'.packages.kernel_elf_debug;
        inherit pkgs;
      };

      run = self.lib.qemu.buildScript {
        inherit pkgs;
        name = "estros-run";
        inherit (release) efiVars diskImage;
      };
      debugScript = self.lib.qemu.buildScript {
        inherit pkgs;
        name = "estros-debug";
        inherit (debug) efiVars diskImage;
        extraFlags = "-S -s";
      };
    in
    {
      packages = {
        inherit run;
        debug = debugScript;
        default = run;

        krun = pkgs.writeShellScriptBin "krun" ''
          exec nix run .#run -- "$@"
        '';
        kdebug = pkgs.writeShellScriptBin "kdebug" ''
          nix build .#gdb
          ${pkgs.alacritty}/bin/alacritty -e ./result/bin/gdb &
          gdb_pid=$!
          trap 'kill $gdb_pid 2>/dev/null' EXIT
          exec nix run .#debug -- "$@"
        '';
        kbacon = pkgs.writeShellScriptBin "kbacon" ''
          cd "$(git rev-parse --show-toplevel)/kernel"
          exec bacon -- -Z json-target-spec "$@"
        '';
      } // pkgs.lib.optionalAttrs isLinux {
        gdb = pkgs.writeShellScriptBin "gdb" ''
          kernel_path=$(nix build .#kernel_elf_debug --no-link --print-out-paths)
          init_path=$(nix build .#init_debug --no-link --print-out-paths)
          tmpgdbinit=$(mktemp)
          trap 'rm -f "$tmpgdbinit"' EXIT
          sed \
            -e "s|KERNEL_ELF_PATH|$kernel_path/kernel.elf|g" \
            -e "s|INIT_ELF_PATH|$init_path/init.elf|g" \
            ${./gdbinit} > "$tmpgdbinit"
          exec ${cross.buildPackages.gdb}/bin/aarch64-none-elf-gdb -ix "$tmpgdbinit" "$@"
        '';
      };
    };
}
