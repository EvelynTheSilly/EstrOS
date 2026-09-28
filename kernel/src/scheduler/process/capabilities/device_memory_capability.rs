use crate::{
    dtb::{Dtb, strings_block::StringsBlock, structure_block::Node},
    println,
};
use alloc::vec::Vec;

const REG_TUPLE_BYTES: usize = 16;
const PAGE_SIZE: u64 = 0x1000;

#[derive(Debug, Clone, Copy)]
pub struct DeviceMemCap {
    pub addr: usize,
    pub size: usize,
}

impl DeviceMemCap {
    pub fn from_dtb(dtb: &Dtb) -> Vec<DeviceMemCap> {
        let root = dtb
            .structure
            .root
            .as_ref()
            .expect("dtb structure block has no root node");
        assert_eq!(
            root.prop_u32(&dtb.strings, "#address-cells"),
            Some(2),
            "expected #address-cells of 2 at the root"
        );
        assert_eq!(
            root.prop_u32(&dtb.strings, "#size-cells"),
            Some(2),
            "expected #size-cells of 2 at the root"
        );
        let mut caps = Vec::new();
        Self::walk(root, &dtb.strings, &mut caps);
        caps
    }

    fn walk(node: &Node, strings: &StringsBlock, out: &mut Vec<DeviceMemCap>) {
        for child in &node.children {
            Self::walk(child, strings, out);
        }

        if node.prop(strings, "device_type") == Some(b"memory".as_slice()) {
            return;
        }

        if let Some(reg) = node.prop(strings, "reg") {
            if reg.len() % REG_TUPLE_BYTES != 0 {
                println!(
                    "warn: node {} has a reg of {} bytes, not a whole number of tuples; skipping",
                    node.name,
                    reg.len()
                );
            } else {
                for tuple in reg.chunks_exact(REG_TUPLE_BYTES) {
                    if let Some(cap) = Self::from_tuple(node, tuple) {
                        out.push(cap);
                    }
                }
            }
        }
    }

    fn from_tuple(node: &Node, tuple: &[u8]) -> Option<DeviceMemCap> {
        let addr = u64::from_be_bytes(tuple[..8].try_into().unwrap());
        let size = u64::from_be_bytes(tuple[8..].try_into().unwrap());
        if size == 0 {
            println!(
                "warn: node {} has a zero sized reg region; skipping",
                node.name
            );
            return None;
        }
        if !size.is_power_of_two() {
            println!(
                "warn: node {} has a reg size of {:#x}, not a power of two; skipping",
                node.name, size
            );
            return None;
        }
        if addr % PAGE_SIZE != 0 {
            println!(
                "warn: node {} has a reg address of {:#x}, not page aligned; skipping",
                node.name, addr
            );
            return None;
        }
        Some(DeviceMemCap {
            addr: addr as usize,
            size: size as usize,
        })
    }
}
