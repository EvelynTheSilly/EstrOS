/// interface:
/// x0: vaddr (page aligned)
/// x1: flags bitmask
///     bit 0: EXEC — page is executable by user code, otherwise UXN
///     bit 1: WRITE — page is writable, otherwise READ_ONLY
///     bits 2-63: reserved, must be 0
///     pages are always user-mode, non-global, normal cacheable and
///     deny privileged execution
///
/// errors:
/// 1: unaligned vaddr
/// 2: invalid flags (reserved bits set)
/// 3: mapping failed
///
/// returns 0 on success
use crate::{
    mem::mmu::NORMAL_CACHEABLE,
    scheduler::{
        CpuScheduler, PROCESS_MANAGER,
        process::{Pid, mem_management::MapPageError, threads::Tid},
    },
    syncronisation::Mutex,
    syscall_assert,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};
use aarch64_paging::descriptor::Attributes;

const MAP_PAGE_EXEC: u64 = 1 << 0;
const MAP_PAGE_WRITE: u64 = 1 << 1;
const MAP_PAGE_FLAGS_MASK: u64 = MAP_PAGE_EXEC | MAP_PAGE_WRITE;

fn flags_to_attributes(flags: u64) -> Attributes {
    let mut attrs = NORMAL_CACHEABLE
        | Attributes::PXN
        | Attributes::UXN
        | Attributes::USER
        | Attributes::VALID
        | Attributes::ACCESSED
        | Attributes::NON_GLOBAL;
    if flags & MAP_PAGE_EXEC != 0 {
        attrs.remove(Attributes::UXN);
    }
    if flags & MAP_PAGE_WRITE == 0 {
        attrs |= Attributes::READ_ONLY;
    }
    attrs
}

pub fn map_page(state: &mut State, pid: Pid, _tid: Tid) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        let vaddr = state.x[0] as usize;
        let flags = state.x[1];

        let Ok(proc) = scheduler.get_process_mut(pid) else {
            return None;
        };

        syscall_assert!(vaddr % 4096 == 0, 1);
        syscall_assert!(flags & !MAP_PAGE_FLAGS_MASK == 0, 2);

        let attributes = flags_to_attributes(flags);

        match proc.mem_manager.map_page(vaddr, attributes) {
            Ok(()) => Some(Ok(0)),
            Err(MapPageError::VaNotPageAligned(_)) => syscall_err(1),
            Err(MapPageError::MapError(_)) => syscall_err(3),
        }
    })
}
