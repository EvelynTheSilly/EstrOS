{...}: let
  buildDiskImage = {
    init,
    kernel,
    pkgs,
    limine ? pkgs.limine-full,
    ovmf ? pkgs.pkgsCross.aarch64-multiplatform.OVMF.fd,
  }: let
    limineConf = ./limine.conf;
    bootloaderSettings = ./bootloader_settings.json;

    stage1 = pkgs.runCommand "estros-disk-image-stage1" {} ''
      mkdir -p $out

      dd if=/dev/zero of=$out/disk.img bs=1M count=64

      ${pkgs.gptfdisk}/bin/sgdisk -o $out/disk.img
      ${pkgs.gptfdisk}/bin/sgdisk -n 1:2048:0 -t 1:ef00 $out/disk.img

      dd if=/dev/zero of=$out/part.fat bs=1M count=63
      ${pkgs.dosfstools}/bin/mkfs.vfat -F 32 $out/part.fat
      ${pkgs.mtools}/bin/mmd -i $out/part.fat ::/EFI
      ${pkgs.mtools}/bin/mmd -i $out/part.fat ::/EFI/BOOT
      ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${limine}/share/limine/BOOTAA64.EFI ::/EFI/BOOT/BOOTAA64.EFI
      ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${limineConf} ::/limine.conf

      dd if=$out/part.fat of=$out/disk.img bs=1M seek=1 conv=notrunc

      ${pkgs.gptfdisk}/bin/sgdisk -e $out/disk.img

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

    stage2 = pkgs.runCommand "estros-disk-image-stage2" {} ''
      mkdir -p $out

      cp ${stage1}/part.fat $out/part.fat
      chmod +w $out/part.fat
      ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${kernel}/kernel.elf ::/kernel.elf

      cp ${stage1}/disk.img $out/disk.img
      chmod +w $out/disk.img
      dd if=$out/part.fat of=$out/disk.img bs=1M seek=1 conv=notrunc

      ${pkgs.gptfdisk}/bin/sgdisk -e $out/disk.img
    '';

    diskImage = pkgs.runCommand "estros-disk-image-stage3" {} ''
      mkdir -p $out

      cp ${stage2}/part.fat $out/part.fat
      chmod +w $out/part.fat
      ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${init}/init.elf ::/init.elf

      cp ${stage2}/disk.img $out/disk.img
      chmod +w $out/disk.img
      dd if=$out/part.fat of=$out/disk.img bs=1M seek=1 conv=notrunc

      ${pkgs.gptfdisk}/bin/sgdisk -e $out/disk.img
    '';

    efiVars = stage1;
  in {
    inherit diskImage efiVars;
  };
in {
  flake = {...}: {
    lib = {
      qemu = {
        inherit buildDiskImage;
      };
    };
  };
}
