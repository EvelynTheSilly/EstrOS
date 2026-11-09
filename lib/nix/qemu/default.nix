{
  inputs,
  self,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  perSystem = {
    self',
    pkgs,
    system,
    ...
  }: let
    cross = pkgs.pkgsCross.aarch64-embedded;
    isLinux = system != "aarch64-darwin";
    inits = self.estros.inits;
    init = inits.rust_init;
    releaseInit = self.lib.buildInit {init = init.release;};
    debugInit = self.lib.buildInit {init = init.debug;};

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

    mkImage = init: kernel:
      self.lib.qemu.buildDiskImage {
        inherit init kernel pkgs;
      };

    # Disk with the kernel but no init.elf: krun --init-file splices the ELF
    # in at runtime instead of baking it in at build time.
    mkBareImage = kernel:
      self.lib.qemu.buildDiskImage {
        init = null;
        inherit kernel pkgs;
        withInit = false;
      };

    # Halts under gdb's control: -S freezes the guest, -s opens :1234.
    mkGdbRun = {
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

    bareRelease = mkBareImage releaseKernel;
    bareDebug = mkBareImage debugKernel;

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
    # Backend for krun --init-file: the wrapper swaps in a custom init.elf by
    # pointing ESTROS_DISK_IMAGE at a patched copy of the bare disk.
    run-bare = self.lib.qemu.buildScript {
      inherit pkgs;
      name = "estros-run-bare";
      inherit (bareRelease) efiVars diskImage;
    };
    run-bare-debug = self.lib.qemu.buildScript {
      inherit pkgs;
      name = "estros-run-bare-debug";
      inherit (bareDebug) efiVars diskImage;
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
  in {
    packages =
      {
        inherit run run-debug run-bare run-bare-debug run-gdb run-gdb-kernel-release run-gdb-init-release run-gdb-release;
        init_debug = debugInit;
        init_release = releaseInit;
        default = run;

        krun = pkgs.writeShellScriptBin "krun" ''
          run_pkg=run
          init_file=""
          args=()
          while [[ $# -gt 0 ]]; do
            case "$1" in
              --debug)
                run_pkg=run-debug
                shift
                ;;
              --init-file)
                if [[ $# -lt 2 ]]; then
                  echo "krun: --init-file requires a path" >&2
                  exit 1
                fi
                init_file="$2"
                shift 2
                ;;
              --init-file=*)
                init_file="''${1#--init-file=}"
                shift
                ;;
              *)
                args+=("$1")
                shift
                ;;
            esac
          done

  if [[ -n "$init_file" ]]; then
    if [[ ! -f "$init_file" ]]; then
      echo "krun: init file not found: $init_file" >&2
      exit 1
    fi
    if [[ "$run_pkg" == run ]]; then
      run_pkg=run-bare
    else
      run_pkg=run-bare-debug
    fi
    bare_bin=$(nix build ".#$run_pkg" --no-link --print-out-paths)
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    cp "$bare_bin/disk.img" "$tmpdir/disk.img"
    chmod +w "$tmpdir/disk.img"
    ${pkgs.mtools}/bin/mcopy -i "$tmpdir/disk.img@@1048576" -- "$init_file" ::/init.elf
    export ESTROS_DISK_IMAGE="$tmpdir/disk.img"
    exec "$bare_bin/bin/estros-$run_pkg" "''${args[@]}"
  fi

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
      }
      // pkgs.lib.optionalAttrs isLinux {
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
