#![no_std]
#![no_main]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};

#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() {
    naked_asm!(
        "
        ldr x0, =_stack
        mov sp, x0
        bl {}
        b .
        ",
        sym main
    );
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
const HELLO_FROM_RUST: &str = "haiii, from rust this time";

fn main() {
    unsafe {
        asm!(
            "
            svc #1
            ",
            in("x0") HELLO_FROM_RUST.as_ptr(),
            in("x1") HELLO_FROM_RUST.len()
        );
    }
}
