use alloc::vec::Vec;

#[derive(Debug)]
pub struct StringsBlock {
    data: Vec<u8>,
}

impl StringsBlock {
    pub fn new(base: *const u8, size: u32) -> StringsBlock {
        let size = size as usize;
        let mut data = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                data.push(*base.add(i));
            }
        }
        StringsBlock { data }
    }

    pub fn get_string(&self, offset: u32) -> Option<&str> {
        let start = offset as usize;
        if start >= self.data.len() {
            return None;
        }
        let end = start + self.data[start..].iter().position(|&b| b == 0)?;
        core::str::from_utf8(&self.data[start..end]).ok()
    }
}
