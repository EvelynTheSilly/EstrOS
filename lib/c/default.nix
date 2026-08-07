{ pkgs, ... }:
let
  cross = pkgs.pkgsCross.aarch64-embedded;
  sysroot = ./sysroot;

  linker-script = ./linker.ld;

  estros-libc =
    pkgs.runCommand "estros-libc"
      {
        src = ./libs;
        gcc = cross.buildPackages.gcc.cc;
        binutils = cross.buildPackages.binutils;
      }
      ''
        mkdir -p $out/lib
        CFLAGS="-march=armv8-a -ffreestanding -nostdlib -fno-builtin -isystem ${sysroot}/include"

        for f in $src/crt.c $src/crt.S; do
          [ -e "$f" ] || continue
          $gcc/bin/aarch64-none-elf-gcc $CFLAGS -c "$f" -o $out/lib/crt0.o
        done

        objs=""
        for f in $src/*.c $src/*.S; do
          [ -e "$f" ] || continue
          case "$(basename "$f")" in
            ctr.c|ctr.S) continue ;;
          esac
          obj="$out/lib/$(basename "$f").o"
          $gcc/bin/aarch64-none-elf-gcc $CFLAGS -c "$f" -o "$obj"
          objs="$objs $obj"
        done
        $binutils/bin/aarch64-none-elf-ar rcs $out/lib/libc.a $objs
        ln -s libc.a $out/lib/libestros.a
      '';

  estros-gcc = cross.buildPackages.wrapCCWith {
    cc = cross.buildPackages.gcc.cc;
    bintools = cross.buildPackages.binutils;
    extraTools = with cross.buildPackages; [
      binutils
      binutils-unwrapped
      elfutils
    ];
    extraBuildCommands = ''
      echo "-B ${estros-libc}/lib -L${estros-libc}/lib" >> $out/nix-support/cc-cflags-before
      echo "-isystem ${sysroot}/include" >> $out/nix-support/cc-cflags
      echo "-static -T${linker-script}" >> $out/nix-support/cc-cflags
    '';
  };

  aarch64-estros-binutils =
    pkgs.runCommandLocal "aarch64-estros-binutils"
      {
        wrapped = estros-gcc;
      }
      ''
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
  packages.estros-gcc = estros-gcc;
  packages.estros-libc = estros-libc;
  packages.aarch64-estros-binutils = aarch64-estros-binutils;
}
