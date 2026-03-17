qemuflags := " -M virt \
    -cpu cortex-a57 \
    -drive if=pflash,unit=0,format=raw,file=bin/AAVMF_CODE.fd,readonly=on \
    -drive if=pflash,unit=1,format=raw,file=bin/AAVMF_VARS.fd \
    -drive file=build/disk.img,format=raw \
    -serial mon:stdio \
    -device ramfb \
	-device qemu-xhci \
	-device usb-kbd \
	-device usb-mouse \
	-smp 4 \
    -semihosting \
"

build_kernel_elf opt="debug":
    if [ -f "config.sh" ]; then \
    source ./config.sh; \
    else \
    source ./exampleconfig.sh; \
    fi 

    cargo build --bin kernel $( [ {{ opt }} = release ] && printf '%s' --release )
    cp ./target/aarch64-none-custom/debug/kernel ./build/kernel.elf

build_img:
    # Create a clean 64MB file
    dd if=/dev/zero of=build/disk.img bs=1M count=64

    # Create the GPT table and the partition
    sgdisk -o build/disk.img
    sgdisk -n 1:2048:0 -t 1:ef00 build/disk.img

    # 3. Create the FAT partition in a separate file (63MB)
    dd if=/dev/zero of=build/part.fat bs=1M count=63
    mkfs.vfat -F 32 build/part.fat

    # 4. Fill the partition
    mmd -i build/part.fat ::/EFI
    mmd -i build/part.fat ::/EFI/BOOT
    mcopy -i build/part.fat bin/BOOTAA64.EFI ::/EFI/BOOT/BOOTAA64.EFI
    mcopy -i build/part.fat build/kernel.elf ::/kernel.elf
    mcopy -i build/part.fat limine.conf ::/limine.conf

    # 5. Stitch it together
    dd if=build/part.fat of=build/disk.img bs=1M seek=1 conv=notrunc

    # Relocate backup headers and verify
    # -e: moves the backup GPT header to the actual end of the 64MB file
    sgdisk -e build/disk.img

build_init opt="debug":
    if [ -f "config.sh" ]; then \
    source ./config.sh;echo "found $INIT"; \
    else \
    source ./exampleconfig.sh; \
    fi; \
    cargo build --bin $INIT $( [ {{ opt }} = release ] && printf '%s' --release ); \
    cp ./target/aarch64-none-custom/{{ opt }}/$INIT ./build/init.elf

build opt="debug":
    just create_temp_dir ./bin
    just create_temp_dir ./build
    just build_init
    just build_kernel_elf {{ opt }}
    just get_binary_blobs
    just build_img

get_binary_blobs:
    if [ -f "config.sh" ]; then \
    source ./config.sh; \
    else \
    source ./exampleconfig.sh; \
    fi 
    cp $LIMINE_EFI_PATH ./bin/BOOTAA64.EFI
    cp $BOOT_FIRMWARE_PATH/AAVMF_CODE.fd ./bin/AAVMF_CODE.fd
    cp $BOOT_FIRMWARE_PATH/AAVMF_VARS.fd ./bin/AAVMF_VARS.fd
    chmod +w ./bin/AAVMF_CODE.fd
    truncate -s 64M bin/AAVMF_CODE.fd
    chmod +w ./bin/AAVMF_VARS.fd
    truncate -s 64M bin/AAVMF_VARS.fd
    virt-fw-vars --input bin/AAVMF_VARS.fd --set-json bootloader_settings.json --output bin/AAVMF_VARS.fd

run:
    @echo "running vm"
    @echo "exit with ctrl a, then x"
    @echo ""
    @echo ""
    qemu-system-aarch64 {{ qemuflags }}

buildrun opt="debug":
    just build {{ opt }}
    just run

debug:
    #! /usr/bin/env nix-shell
    #! nix-shell -i bash -p bash
    alacritty -e bash -c "just gdb" &
    echo "running vm"
    echo ""
    echo ""
    echo ""
    qemu-system-aarch64 {{ qemuflags }} -S -s
    kill $!

gdb:
    aarch64-none-elf-gdb -ex "target remote :1234" 

fulldebug opt="debug":
    #! /usr/bin/env nix-shell
    #! nix-shell -i bash -p bash
    just build {{ opt }}
    alacritty -e bash -c "just gdb" &
    echo "running vm"
    echo ""
    echo ""
    echo ""
    qemu-system-aarch64 {{ qemuflags }} -S -s
    kill $!

create_temp_dir name:
    mkdir -p {{ name }}
    rm {{ name }} -rf 
    mkdir -p {{ name }}
