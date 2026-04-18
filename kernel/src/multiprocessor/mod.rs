use crate::println;
use anyhow::{Result, anyhow};
use core::{arch::naked_asm, hint::spin_loop};
use limine::{mp::Cpu, request::MpRequest};

#[used]
#[unsafe(link_section = ".requests")]
static PROCESSORS: MpRequest = MpRequest::new();

#[unsafe(naked)]
unsafe extern "C" fn core_entry(cpu: &Cpu) -> ! {
    naked_asm!(
        "
        bl core_init
        "
    );
}

pub fn mp_init() -> Result<()> {
    println!("setting up multiprocessing...");
    PROCESSORS
        .get_response()
        .ok_or(anyhow!("no processors responce"))?
        .cpus()
        .iter()
        .for_each(|cpu| {
            cpu.goto_address.write(core_entry);
        });
    println!("set up multiprocessing");
    Ok(())
}

#[allow(unused)] // its used in the assembly
#[unsafe(no_mangle)]
unsafe extern "C" fn core_init(cpu: &Cpu) -> ! {
    unsafe {
        core::ptr::write_volatile(0xFFFF_0000_0900_0000 as *mut u8, 67);
    }
    println!("cpu init: {:#?}", cpu.id);
    loop {
        spin_loop();
    }
}
