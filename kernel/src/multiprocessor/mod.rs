use crate::println;
use anyhow::{Result, anyhow};
use core::{arch::naked_asm, hint::spin_loop};
use limine::{mp::MpInfo, request::MpRequest};

#[used]
#[unsafe(link_section = ".requests")]
static PROCESSORS: MpRequest = MpRequest::new(0);

#[unsafe(naked)]
unsafe extern "C" fn core_entry(cpu: &MpInfo) -> ! {
    naked_asm!(
        "
        bl {}
        ", 
        sym core_init
    );
}

pub fn mp_init() -> Result<()> {
    println!("setting up multiprocessing...");
    PROCESSORS
        .response()
        .ok_or(anyhow!("no processors responce"))?
        .cpus()
        .iter()
        .for_each(|cpu| {
            cpu.bootstrap(core_entry, 67);
        });
    println!("set up multiprocessing");
    Ok(())
}

#[allow(unused)] // its used in the assembly
unsafe extern "C" fn core_init(cpu: &MpInfo) -> ! {
    unsafe {
        core::ptr::write_volatile(0xFFFF_0000_0900_0000 as *mut u8, 67);
    }
    println!("cpu init: {:#?}", cpu.processor_id);
    loop {
        spin_loop();
    }
}
