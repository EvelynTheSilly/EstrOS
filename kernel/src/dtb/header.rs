use anyhow::Result;
use core::ptr;

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct DtbHeader {
    magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_structs: u32,
}

impl DtbHeader {
    pub unsafe fn new(base: *const u8) -> DtbHeader {
        let wrong_endianness_header: DtbHeader =
            unsafe { ptr::read_volatile(base as *const DtbHeader) };
        DtbHeader {
            magic: u32::from_be(wrong_endianness_header.magic),
            totalsize: u32::from_be(wrong_endianness_header.totalsize),
            off_dt_struct: u32::from_be(wrong_endianness_header.off_dt_struct),
            off_dt_strings: u32::from_be(wrong_endianness_header.off_dt_strings),
            off_mem_rsvmap: u32::from_be(wrong_endianness_header.off_mem_rsvmap),
            version: u32::from_be(wrong_endianness_header.version),
            last_comp_version: u32::from_be(wrong_endianness_header.last_comp_version),
            boot_cpuid_phys: u32::from_be(wrong_endianness_header.boot_cpuid_phys),
            size_dt_strings: u32::from_be(wrong_endianness_header.size_dt_strings),
            size_dt_structs: u32::from_be(wrong_endianness_header.size_dt_structs),
        }
    }
    pub fn is_sane(self: &Self) -> Result<()> {
        // magic check
        anyhow::ensure!(self.magic == 0xd00dfeed, "invalid header magic");
        anyhow::ensure!(self.version == 17, "dtb version is not v17");
        Ok(())
    }
}
