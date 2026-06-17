use aarch64_paging::descriptor::Attributes;
use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::{KERNEL_PHYS_BASE, println};

pub const NORMAL_CACHEABLE: Attributes =
    Attributes::ATTRIBUTE_INDEX_0.union(Attributes::INNER_SHAREABLE);

pub fn init_mmu() {
    let mut par: u64;
    unsafe {
        asm!(
            "at s1e1r, {va}",
            "mrs {par}, par_el1",
            va = in(reg) 0xFFFFFFFF80000000u64,
            par = out(reg) par,
        );
    }
    let phys_base = if par & 1 == 0 {
        par & 0x0000FFFFFFFFFFF000
    } else {
        println!("AT translation failed, using default base");
        0x40000000u64
    };
    KERNEL_PHYS_BASE.store(phys_base, Ordering::Relaxed);
}
