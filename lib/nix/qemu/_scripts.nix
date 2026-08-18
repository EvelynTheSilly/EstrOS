{ pkgs, name, efiVars, diskImage, extraFlags ? "" }:

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
''
