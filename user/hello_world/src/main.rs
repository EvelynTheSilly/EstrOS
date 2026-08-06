#![no_std]
#![no_main]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};
use estrogen::{println, syscall};

#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() {
    naked_asm!(
        "
        ldr x0, =_stack
        mov sp, x0
        bl {}
        svc #2
        b . // fallback
        ",
        sym main
    );
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    let line1 = "haiii";
    let line2 = "this is userspace code";
    let line3 = "it can print to the console";
    let line4 = "im gonna exit now o/";
    let buf = [0; 64];

    syscall!(3, 0, buf.as_ptr(), buf.len());
    syscall!(1, buf.as_ptr(), buf.len());
    syscall!(1, line1.as_ptr(), line1.len());
    syscall!(1, line2.as_ptr(), line2.len());
    syscall!(1, line3.as_ptr(), line3.len());
    println!("it can even {}", "print formatted");
    syscall!(1, line4.as_ptr(), line4.len());
    syscall!(2);
}
