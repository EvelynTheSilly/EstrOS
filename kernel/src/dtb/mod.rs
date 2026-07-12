#![allow(unused)]

use core::fmt::Display;

use crate::dtb::header::DtbHeader;
use crate::dtb::memory_reservation_block::MemoryReservationBlock;
use crate::dtb::strings_block::StringsBlock;
use crate::dtb::structure_block::StructureBlock;
use crate::println;
use anyhow::Result;
use core::fmt;

mod header;
mod memory_reservation_block;
mod strings_block;
mod structure_block;

pub struct Dtb {
    header: DtbHeader,
    memory_reservations: MemoryReservationBlock,
    strings: StringsBlock,
    structure: StructureBlock,
}

impl Dtb {
    pub fn new(base: *const u8) -> Result<Dtb> {
        let header = unsafe { DtbHeader::new(base) };
        println!("header {:?}", header);
        // header can be concidered sane after this
        header.is_sane()?;
        let mem_base;
        let struct_base;
        let strings_base;
        unsafe {
            mem_base = base.add(header.off_mem_rsvmap as usize);
            struct_base = base.add(header.off_dt_struct as usize);
            strings_base = base.add(header.off_dt_strings as usize);
        }
        let memory_reservations = MemoryReservationBlock::new(mem_base);
        let strings = StringsBlock::new(strings_base, header.size_dt_strings);
        let structure = StructureBlock::new(struct_base);
        Ok(Dtb {
            header,
            memory_reservations,
            strings,
            structure,
        })
    }
}

impl fmt::Display for Dtb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let totalsize = self.header.totalsize;
        let version = self.header.version;
        let off_dt_struct = self.header.off_dt_struct;
        let size_dt_structs = self.header.size_dt_structs;
        let off_dt_strings = self.header.off_dt_strings;
        let size_dt_strings = self.header.size_dt_strings;
        let off_mem_rsvmap = self.header.off_mem_rsvmap;
        let boot_cpuid_phys = self.header.boot_cpuid_phys;
        writeln!(f, "=== Device Tree Blob ===")?;
        writeln!(f, "  total size:      {} bytes", totalsize)?;
        writeln!(f, "  version:         {}", version)?;
        writeln!(f, "  struct offset:   0x{:X}", off_dt_struct)?;
        writeln!(f, "  struct size:     0x{:X}", size_dt_structs)?;
        writeln!(f, "  strings offset:  0x{:X}", off_dt_strings)?;
        writeln!(f, "  strings size:    0x{:X}", size_dt_strings)?;
        writeln!(f, "  mem rsvmap off:  0x{:X}", off_mem_rsvmap)?;
        writeln!(f, "  boot cpuid phys: 0x{:X}", boot_cpuid_phys)?;
        writeln!(f, "")?;
        if let Some(ref root) = self.structure.root {
            root.fmt_with_strings(f, &self.strings, 0)
        } else {
            writeln!(f, "(no root node)")
        }
    }
}
