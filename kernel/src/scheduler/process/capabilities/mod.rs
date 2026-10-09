use crate::{
    dtb::Dtb, println, scheduler::process::capabilities::{
        device_memory_capability::DeviceMemCap,
        message_reciever_capability::ProcessMessageCapability,
    },
};
use alloc::{collections::BTreeMap};

pub mod device_memory_capability;
pub mod message_reciever_capability;

pub enum Capability {
    /// rights to the memory region associated with some device
    DeviceMemCapability(DeviceMemCap),
    /// rights to a proccesses reciever channel
    RecieverCapability(ProcessMessageCapability),
}

impl Capability {
    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        let type = u64::from_be_bytes(bytes[0..8])
        println!("DEBUG: type {}", type);
        todo!()
    }
}

pub type CapID = u64;

#[derive(Default)]
pub struct CapStore {
    rolling_cap_id: CapID,
    pub caps: BTreeMap<CapID, Capability>,
}

impl CapStore {
    fn new_cap_id(&mut self) -> CapID {
        let id = self.rolling_cap_id;
        self.rolling_cap_id += 1;
        id
    }
    pub fn insert_capability(&mut self, capability: Capability) -> CapID {
        let id = self.new_cap_id();
        self.caps.insert(id, capability);
        id
    }
    pub fn populate_mem(&mut self, dtb: &Dtb) {
        for cap in DeviceMemCap::from_dtb(dtb) {
            self.insert_capability(Capability::DeviceMemCapability(cap));
        }
    }
}
