#![no_std]
#![no_main]
#![feature(macro_metavar_expr_concat)]
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![allow(unused_unsafe)]
#![allow(
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc
)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::float_equality_without_abs)]
#![warn(clippy::missing_const_for_fn)]

use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};
use aarch64_cpu::asm::wfi;
use core::{hint::spin_loop, panic::PanicInfo};
use elf::{ElfBytes, endian::AnyEndian};
use limine::{
    BaseRevision,
    mp::Cpu,
    request::{HhdmRequest, MpRequest, RequestsEndMarker, RequestsStartMarker, StackSizeRequest},
};

mod boot;
mod drivers;
mod dtb;
mod irqs;
mod mem;
mod rng;
mod scheduler;
mod syncronisation;
mod uart;
mod vectors;
extern crate alloc;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static PROCESSORS: MpRequest = MpRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static STACK: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

#[used]
#[unsafe(link_section = ".requests")]
static HDDM: HhdmRequest = HhdmRequest::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", { info.message() });
    loop {
        wfi();
    }
}

#[unsafe(no_mangle)]
#[allow(unreachable_code)]
pub extern "C" fn kernel_init() {
    unsafe {
        println!("booting estros...");

        println!("loading init...");
        let init = include_bytes!("../../build/init.elf");
        let init_elf = ElfBytes::<AnyEndian>::minimal_parse(init).expect("INVALID INIT FILE");
        println!("launching process");
        let init_pid = PROCESS_MANAGER
            .lock(|manager| manager.launch_process(init_elf))
            .expect("failed to launch init");
        println!("launched pid {}", init_pid);
    };
}

extern "C" fn get_init_process(initial_thread_state: *mut State) {
    unsafe {
        let thread = PROCESS_MANAGER.lock(|manager| manager.schedule().unwrap());
        *initial_thread_state = thread.state;
    }
    println!("loaded init");
}

#[allow(unused)]
unsafe extern "C" fn core_init(cpu: &Cpu) -> ! {
    unsafe {
        core::ptr::write_volatile(0xFFFF_0000_0900_0000 as *mut u8, 67);
    }
    println!("cpu init: {:#?}", cpu.id);
    loop {
        spin_loop();
    }
}
