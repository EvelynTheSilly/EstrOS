use crate::{
    dtb::memory_reservation_block::MemoryReservationBlock,
    scheduler::process::capabilities::device_memory_capability::DeviceMemCap,
};
use alloc::collections::BTreeMap;

pub mod device_memory_capability;

pub type CapID = u64;

#[derive(Default)]
pub struct CapStore {
    rolling_cap_id: CapID,
    dev_mem_caps: BTreeMap<CapID, DeviceMemCap>,
}

impl CapStore {
    fn new_cap_id(&mut self) -> CapID {
        let id = self.rolling_cap_id;
        self.rolling_cap_id += 1;
        return id;
    }
    fn insert_dev_mem_capability(&mut self, capability: DeviceMemCap) {
        let id = self.new_cap_id();
        self.dev_mem_caps.insert(id, capability);
    }
    pub fn populate_mem(&mut self, device_memories: &MemoryReservationBlock) {
        for entry in &device_memories.entries {
            self.insert_dev_mem_capability(DeviceMemCap {
                block: entry.clone(),
            });
        }
    }
}
