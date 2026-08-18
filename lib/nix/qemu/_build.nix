{ pkgs, kernel, limine, ovmf }:

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
    ${pkgs.mtools}/bin/mcopy -i $out/part.fat ${./limine.conf} ::/limine.conf

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
      --set-json ${./bootloader_settings.json} \
      --output $out/AAVMF_VARS.fd
  '';

in
{
  inherit diskImage efiVars;
}
