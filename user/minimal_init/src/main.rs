#![no_std]
#![no_main]

use core::{arch::naked_asm, panic::PanicInfo};

#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() {
    naked_asm!(
        "
        adr x0, _hello
        mov x1, #23
        svc #1
        b .
    _hello:
        .ascii \"\nHello from userspace!\\n\"
        ",
    );
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
