use alloc::{collections::BTreeMap, vec::Vec};

pub type Mid = u64;

#[derive(Default)]
pub(super) struct MessageStore {
    next_mid: Mid,
    data: BTreeMap<Mid, Message>,
}

pub(super) struct Message {
    data: Vec<u8>,
    read_pointer: usize,
}

impl MessageStore {
    fn push_message(&mut self, message: Message) {
        self.data.insert(self.next_mid, message);
        self.next_mid += 1;
    }
}
