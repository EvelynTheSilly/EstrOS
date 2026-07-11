use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::dtb::strings_block::StringsBlock;

const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

#[derive(Debug)]
pub struct Property {
    pub name_off: u32,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub properties: Vec<Property>,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct StructureBlock {
    pub root: Option<Node>,
}

impl Node {
    pub(crate) fn fmt_with_strings(
        &self,
        f: &mut fmt::Formatter<'_>,
        strings: &StringsBlock,
        indent: usize,
    ) -> fmt::Result {
        let pad = "  ".repeat(indent);
        writeln!(f, "{}node: {}", pad, self.name)?;
        for prop in &self.properties {
            let name = strings
                .get_string(prop.name_off)
                .unwrap_or("<unknown>");
            let value_str = core::str::from_utf8(&prop.value)
                .map(|s| format!("\"{}\"", s))
                .unwrap_or_else(|_| {
                    let hex: String = prop
                        .value
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join("");
                    format!("0x{}", hex)
                });
            writeln!(f, "{}  {} = {}", pad, name, value_str)?;
        }
        for child in &self.children {
            child.fmt_with_strings(f, strings, indent + 1)?;
        }
        Ok(())
    }
}

impl StructureBlock {
    pub fn new(base: *const u8) -> Self {
        let mut offset: usize = 0;
        let root = Self::parse_node(base, &mut offset);
        StructureBlock { root }
    }

    fn parse_node(base: *const u8, offset: &mut usize) -> Option<Node> {
        if Self::read_u32(base, *offset) != FDT_BEGIN_NODE {
            return None;
        }
        *offset += 4;

        let name = Self::read_string(base, offset);
        let mut properties = Vec::new();
        let mut children = Vec::new();

        loop {
            match Self::read_u32(base, *offset) {
                FDT_PROP => {
                    *offset += 4;
                    let len = Self::read_u32(base, *offset) as usize;
                    *offset += 4;
                    let name_off = Self::read_u32(base, *offset);
                    *offset += 4;
                    let mut value = Vec::with_capacity(len);
                    unsafe {
                        for i in 0..len {
                            value.push(*base.add(*offset + i));
                        }
                    }
                    *offset += len;
                    *offset = (*offset + 3) & !3;
                    properties.push(Property { name_off, value });
                }
                FDT_BEGIN_NODE => {
                    if let Some(child) = Self::parse_node(base, offset) {
                        children.push(child);
                    }
                }
                FDT_END_NODE => {
                    *offset += 4;
                    return Some(Node {
                        name,
                        properties,
                        children,
                    });
                }
                FDT_NOP => {
                    *offset += 4;
                }
                FDT_END => {
                    return None;
                }
                _ => {
                    *offset += 4;
                }
            }
        }
    }

    fn read_u32(base: *const u8, offset: usize) -> u32 {
        unsafe { u32::from_be(*(base.add(offset) as *const u32)) }
    }

    fn read_string(base: *const u8, offset: &mut usize) -> String {
        let mut s = String::new();
        unsafe {
            loop {
                let c = *base.add(*offset);
                if c == 0 {
                    *offset += 1;
                    break;
                }
                s.push(c as char);
                *offset += 1;
            }
        }
        *offset = (*offset + 3) & !3;
        s
    }
}
