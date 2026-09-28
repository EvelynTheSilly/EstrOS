use crate::scheduler::process::Process;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use limine::modules::InternalModule;
use limine::request::ModuleRequest;

#[used]
#[unsafe(link_section = ".requests")]
static INIT: ModuleRequest =
    ModuleRequest::new().with_internal_modules(&[&InternalModule::new().with_path(c"/init.elf")]);

pub fn get_init() -> Process {
    let res = INIT.get_response().unwrap();
    let init_bytes = res
        .modules()
        .iter()
        .next()
        .map(|file| unsafe { core::slice::from_raw_parts(file.addr(), file.size() as usize) });

    let init_bytes =
        init_bytes.expect("please provide an init file named init.elf in the boot directory");

    let init_elf = ElfBytes::<AnyEndian>::minimal_parse(init_bytes).expect("INVALID INIT FILE");
    Process::from_elf(init_elf).expect("failed to parse init process")
}
