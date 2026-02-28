//! mmu implementation using aarch64-paging crate
#![allow(unused)]

#[allow(dead_code)]
const KILOBYTE: u64 = 1048;
#[allow(dead_code)]
const MEGABYTE: u64 = KILOBYTE * 1048;
#[allow(dead_code)]
const GIGABYTE: u64 = MEGABYTE * 1048;

const NORMAL_CACHEABLE: Attributes =
    Attributes::ATTRIBUTE_INDEX_0.union(Attributes::INNER_SHAREABLE);
const DEVICE_MEM: Attributes = Attributes::ATTRIBUTE_INDEX_1
    .union(Attributes::ACCESSED)
    .union(Attributes::VALID)
    .union(Attributes::NON_GLOBAL); // Device memory usually shouldn't be Global anyway, but acceptable here.

const MAIR: u64 = (0xFF << 0) | (0x00 << 8);

use aarch64_paging::{
    descriptor::Attributes,
    idmap::IdMap,
    paging::{MemoryRegion, TranslationRegime},
};
use alloc::vec::Vec;
use core::arch::asm;

use crate::println;

pub fn init_mmu(device_ranges: Vec<&MemoryRegion>) -> IdMap {
    let mut memmap = IdMap::new(
        0, //
        0, // 48 bit vadresses
        TranslationRegime::El1And0,
    );
    memmap
        .map_range(
            &MemoryRegion::new(0x40000000, 0x44000000),
            NORMAL_CACHEABLE
                | Attributes::NON_GLOBAL
                | Attributes::VALID
                | Attributes::ACCESSED
                | Attributes::OUTER_SHAREABLE,
        )
        .expect("failed to map kernel");
    for range in device_ranges {
        memmap
            .map_range(range, DEVICE_MEM)
            .expect("failed to map uart");
    }
    unsafe {
        memmap.activate();
        asm!(
            "msr tcr_el1, {val}",
            val = in(reg) 0x00000000B5103510 as u64,
        );
        println!("loaded tcr_el1");
        asm!(
            "msr mair_el1, {val}",
            val = in(reg) MAIR as u64,
        );
        println!("loaded main_el1");
        let mut sctlr: u64;
        asm!(
            "
            mrs x0, sctlr_el1
            ",
            out("x0") sctlr
        );
        println!("read out sctlr: {:b}", sctlr);
        // IMPORTANT: Ensure page table writes are visible and TLB is clean
        asm!("dsb ish"); // Data Synchronization Barrier (Inner Shareable)
        asm!("tlbi vmalle1"); // Invalidate local TLB (just in case)
        asm!("dsb ish"); // Ensure invalidation completes
        asm!("isb"); // Instruction Synchronization Barrier
        asm!(
            "
            orr {val}, {val}, #1        // set M
            msr sctlr_el1, {val}
            isb
            ",
            val = inout(reg) sctlr
        );
        println!("read out sctlr: {:b}", sctlr);
    };

    memmap
}
