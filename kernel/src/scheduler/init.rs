use crate::scheduler::process::Process;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use limine::module::INTERNAL_MODULE_REQUIRED;
use limine::module::InternalModule;
use limine::request::ModulesRequest;

#[used]
#[unsafe(link_section = ".requests")]
static INIT: ModulesRequest =
    ModulesRequest::new_rev1(&[&InternalModule::new(c"/init.elf", c"", INTERNAL_MODULE_REQUIRED)]);

pub fn get_init() -> Process {
    let res = INIT.response().unwrap();
    let init_bytes = res.modules().iter().next().map(|file| file.data());

    let init_bytes =
        init_bytes.expect("please provide an init file named init.elf in the boot directory");

    let init_elf = ElfBytes::<AnyEndian>::minimal_parse(init_bytes).expect("INVALID INIT FILE");
    Process::from_elf(init_elf).expect("failed to parse init process")
}
