{ inputs, ... }:

let
  defineInit =
    {
      name,
      pkg,
    }:
    {
      inherit name pkg;
    };

  buildInit =
    {
      init,
    }:
    init.pkg;

  buildKernel =
    {
      pkgs,
      buildType ? "release",
      rust ? null,
    }:
    let
      rust' = if rust != null then rust else inputs.self.packages.${pkgs.system}.rust;
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust';
    in
    import ../../kernel {
      inherit craneLib pkgs buildType;
      rust = rust';
    };

  limineConf = ./qemu/limine.conf;
  bootloaderSettings = ./qemu/bootloader_settings.json;

  buildDiskImage =
    {
      init,
      kernel,
      pkgs,
      limine ? pkgs.limine-full,
      ovmf ? pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd,
    }:
    let
      diskImage = pkgs.runCommand "estros-disk.img" { } ''
        mkdir -p $out

        dd if=/dev/zero of=$out/disk.img bs=1M count=64

        ${pkgs.gptfdisk}/bin/sgdisk -o $out/disk.img
        ${pkgs.gptfdisk}/bin/sgdisk -n 1:2048:0 -t 1:ef00 $out/disk.img

        dd if=/dev/zero of=$out/part.fat bs=1M count=63
        ${pkgs.dosfstools}/bin/mkfs.vfat -F 32 $out/part.fat
        ${pkgs.mtools}/bin/mmd -i $out/part.fat ::/EFI
        ${pkgs.mtools}/bin/mmd -i $out/part.fat ::/EFI/BOOT
        ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${limine}/share/limine/BOOTAA64.EFI ::/EFI/BOOT/BOOTAA64.EFI
        ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${kernel}/kernel.elf ::/kernel.elf
        ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${init}/init.elf ::/init.elf
        ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${limineConf} ::/limine.conf

        dd if=$out/part.fat of=$out/disk.img bs=1M seek=1 conv=notrunc

        ${pkgs.gptfdisk}/bin/sgdisk -e $out/disk.img
      '';

      efiVars = pkgs.runCommand "efi-vars.fd" { } ''
        mkdir -p $out

        cp ${ovmf}/FV/AAVMF_CODE.fd $out/AAVMF_CODE.fd
        chmod +w $out/AAVMF_CODE.fd
        truncate -s 64M $out/AAVMF_CODE.fd

        cp ${ovmf}/FV/AAVMF_VARS.fd $out/AAVMF_VARS.fd
        chmod +w $out/AAVMF_VARS.fd
        truncate -s 64M $out/AAVMF_VARS.fd
        ${pkgs.python313Packages.virt-firmware}/bin/virt-fw-vars \
          --input $out/AAVMF_VARS.fd \
          --set-json ${bootloaderSettings} \
          --output $out/AAVMF_VARS.fd
      '';
    in
    {
      inherit diskImage efiVars;
    };

  buildScript =
    {
      pkgs,
      name,
      efiVars,
      diskImage,
      extraFlags ? "",
    }:
    pkgs.runCommand name {
      buildInputs = [ pkgs.qemu ];
    } ''
      mkdir -p $out/bin
      cat > $out/bin/${name} <<EOF
      #!${pkgs.bash}/bin/bash
      set -e
      tmpdir=\$(mktemp -d)
      trap 'rm -rf "\$tmpdir"' EXIT
      cp --no-preserve=mode ${efiVars}/AAVMF_CODE.fd "\$tmpdir/AAVMF_CODE.fd"
      cp --no-preserve=mode ${efiVars}/AAVMF_VARS.fd "\$tmpdir/AAVMF_VARS.fd"
      cp --no-preserve=mode ${diskImage}/disk.img "\$tmpdir/disk.img"

      GRAPHICS_FLAGS="-device ramfb"
      EXTRA_FLAGS="${extraFlags}"

      for arg in "\$@"; do
        case "\$arg" in
          --no-graphics)
            GRAPHICS_FLAGS=""
            EXTRA_FLAGS="\$EXTRA_FLAGS -nographic"
            ;;
        esac
      done

      exec qemu-system-aarch64 \\
        -M virt,acpi=off \\
        -cpu cortex-a57 \\
        \$GRAPHICS_FLAGS \\
        -device qemu-xhci \\
        -device usb-kbd \\
        -device usb-mouse \\
        -drive if=pflash,unit=0,format=raw,file="\$tmpdir/AAVMF_CODE.fd",readonly=on \\
        -drive if=pflash,unit=1,format=raw,file="\$tmpdir/AAVMF_VARS.fd" \\
        -drive file="\$tmpdir/disk.img",format=raw \\
        -serial mon:stdio \\
        -semihosting \$EXTRA_FLAGS
      EOF
      chmod +x $out/bin/${name}
    '';
in
{
  flake =
    { ... }:
    {
      lib = {
        inherit defineInit buildInit buildKernel;
        qemu = {
          inherit buildDiskImage buildScript;
        };
      };
    };
}
