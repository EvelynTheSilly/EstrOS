#[derive(Debug)]
#[repr(C)]
pub struct VFSPacketHeader {
    pub magic: u64, // must be b"ESTRVFS\0"
    pub version: u8,
    _pad1: [u8; 3],
    pub packet_type: u8,
    pub packet_version: u8,
    _pad2: [u8; 2],
}

pub const LATEST_SUPPORTED_VERSION: u8 = 0;
pub const MAGIC: u64 = u64::from_le_bytes(*b"ESTRVFS\0");

impl VFSPacketHeader {
    pub fn new(packet_type: u8, packet_version: u8) -> Self {
        VFSPacketHeader {
            magic: MAGIC,
            version: LATEST_SUPPORTED_VERSION,
            _pad1: [0u8; 3],
            packet_type,
            packet_version,
            _pad2: [0u8; 2],
        }
    }
    pub fn as_u8_slice(self) -> Vec<u8> {
        unsafe {
            ::core::slice::from_raw_parts(
                (&self as *const Self) as *const u8,
                ::core::mem::size_of::<VFSPacketHeader>(),
            )
            .to_vec()
        }
    }
}
