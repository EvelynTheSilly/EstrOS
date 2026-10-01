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
      releaseInit = self.lib.buildInit { init = inits.process_spawner.release; };
      debugInit = self.lib.buildInit { init = inits.process_spawner.debug; };

      releaseKernel = self'.packages.kernel_elf;
      debugKernel = self'.packages.kernel_elf_debug;

      kernels = {
        debug = debugKernel;
        release = releaseKernel;
      };
      variantInits = {
        debug = debugInit;
        release = releaseInit;
      };

      mkImage =
        init: kernel:
        self.lib.qemu.buildDiskImage {
          inherit init kernel pkgs;
        };

      # Halts under gdb's control: -S freezes the guest, -s opens :1234.
      mkGdbRun =
        {
          kernelVariant,
          initVariant,
          suffix,
        }:
        self.lib.qemu.buildScript {
          inherit pkgs;
          name = "estros-run-gdb${suffix}";
          inherit (mkImage variantInits.${initVariant} kernels.${kernelVariant}) efiVars diskImage;
          extraFlags = "-S -s";
        };

      release = mkImage releaseInit releaseKernel;
      debug = mkImage debugInit debugKernel;

      run = self.lib.qemu.buildScript {
        inherit pkgs;
        name = "estros-run";
        inherit (release) efiVars diskImage;
      };
      run-debug = self.lib.qemu.buildScript {
        inherit pkgs;
        name = "estros-run-debug";
        inherit (debug) efiVars diskImage;
      };
      run-gdb = mkGdbRun {
        kernelVariant = "debug";
        initVariant = "debug";
        suffix = "";
      };
      run-gdb-kernel-release = mkGdbRun {
        kernelVariant = "release";
        initVariant = "debug";
        suffix = "-kernel-release";
      };
      run-gdb-init-release = mkGdbRun {
        kernelVariant = "debug";
        initVariant = "release";
        suffix = "-init-release";
      };
      run-gdb-release = mkGdbRun {
        kernelVariant = "release";
        initVariant = "release";
        suffix = "-release";
      };
    in
    {
      packages = {
        inherit run run-debug run-gdb run-gdb-kernel-release run-gdb-init-release run-gdb-release;
        init_debug = debugInit;
        init_release = releaseInit;
        default = run;

        krun = pkgs.writeShellScriptBin "krun" ''
          run_pkg=run
          args=()
          for arg in "$@"; do
            if [[ "$arg" == "--debug" ]]; then
              run_pkg=run-debug
            else
              args+=("$arg")
            fi
          done
          exec nix run ".#$run_pkg" -- "''${args[@]}"
        '';
        kdebug = pkgs.writeShellScriptBin "kdebug" ''
          ${self.lib.variantArgs {}}

          case "$kernel_variant-$init_variant" in
            release-release) run_pkg=run-gdb-release ;;
            release-debug) run_pkg=run-gdb-kernel-release ;;
            debug-release) run_pkg=run-gdb-init-release ;;
            *) run_pkg=run-gdb ;;
          esac

          gdb_path=$(nix build .#gdb --no-link --print-out-paths)
          ${pkgs.alacritty}/bin/alacritty -e "$gdb_path/bin/gdb" "''${variant_flags[@]}" "''${variant_args[@]}" &
          gdb_pid=$!
          trap 'kill $gdb_pid 2>/dev/null' EXIT
          nix run ".#$run_pkg" -- "''${variant_args[@]}"
        '';
        kbacon = pkgs.writeShellScriptBin "kbacon" ''
          cd "$(git rev-parse --show-toplevel)/kernel"
          exec bacon -- -Z json-target-spec "$@"
        '';
      } // pkgs.lib.optionalAttrs isLinux {
        gdb = pkgs.writeShellScriptBin "gdb" ''
          ${self.lib.variantArgs {}}

          if [[ "$kernel_variant" == release ]]; then
            kernel_attr=kernel_elf
          else
            kernel_attr=kernel_elf_debug
          fi

          kernel_path=$(nix build ".#$kernel_attr" --no-link --print-out-paths)
          init_path=$(nix build ".#init_$init_variant" --no-link --print-out-paths)
          kernel_src="$PWD/kernel"
          tmpgdbinit=$(mktemp)
          trap 'rm -f "$tmpgdbinit"' EXIT
          sed \
            -e "s|KERNEL_ELF_PATH|$kernel_path/kernel.elf|g" \
            -e "s|INIT_ELF_PATH|$init_path/init.elf|g" \
            -e "s|KERNEL_SRC_PATH|$kernel_src|g" \
            ${./gdbinit} > "$tmpgdbinit"
          exec ${cross.buildPackages.gdb}/bin/aarch64-none-elf-gdb -nx -ix "$tmpgdbinit" "''${variant_args[@]}"
        '';
      };
    };
}
