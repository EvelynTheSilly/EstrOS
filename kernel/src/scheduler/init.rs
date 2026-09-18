use crate::println;
use crate::scheduler::process::Process;
use crate::scheduler::{CpuScheduler, PROCESS_MANAGER};
use crate::syncronisation::Mutex;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use limine::modules::InternalModule;
use limine::request::ModuleRequest;

#[used]
#[unsafe(link_section = ".requests")]
static INIT: ModuleRequest = ModuleRequest::new().with_internal_modules(&[
    &InternalModule::new().with_path(c"/init1.elf"),
    &InternalModule::new().with_path(c"/init2.elf"),
    &InternalModule::new().with_path(c"/init3.elf"),
    &InternalModule::new().with_path(c"/init4.elf"),
    &InternalModule::new().with_path(c"/init5.elf"),
    &InternalModule::new().with_path(c"/init6.elf"),
    &InternalModule::new().with_path(c"/init7.elf"),
    &InternalModule::new().with_path(c"/init8.elf"),
    &InternalModule::new().with_path(c"/init9.elf"),
]);

pub fn launch_init() {
    let res = INIT.get_response();
    let init_files = res
        .unwrap()
        .modules()
        .iter()
        .map(|file| unsafe { core::slice::from_raw_parts(file.addr(), file.size() as usize) });

    assert!(
        init_files.len() >= 1,
        "please provide one or more init files named init1.elf, init2.elf, etc... in the boot directory"
    );

    PROCESS_MANAGER.lock(|manager| {
        init_files.for_each(|bytes| {
            let init_elf = ElfBytes::<AnyEndian>::minimal_parse(bytes).expect("INVALID INIT FILE");
            let init_process = Process::from_elf(init_elf).expect("failed to map init process");
            let init_pid = manager
                .launch_process(init_process)
                .expect("failed to launch init");
            println!("launched init pid: {}", init_pid);
        });
    });
}
