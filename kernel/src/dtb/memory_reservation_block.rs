use crate::println;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct MemoryReservationBlock {
    pub entries: Vec<MemoryReservationBlockEntry>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MemoryReservationBlockEntry {
    pub address: usize,
    pub size: usize,
}

impl MemoryReservationBlock {
    pub fn new(base: *const u8) -> Self {
        let mut entries = vec![];
        let mut counter = base;
        unsafe {
            loop {
                println!("reading cap");
                let entry = MemoryReservationBlockEntry {
                    address: usize::from_be(*(counter as *const usize)),
                    size: usize::from_be(*(counter.add(8) as *const usize)),
                };
                if entry.address == 0 && entry.size == 0 {
                    println!("last cap");
                    break;
                }
                entries.push(entry);
                counter = counter.add(16);
            }
        }
        MemoryReservationBlock { entries: entries }
    }
}
