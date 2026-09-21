use alloc::{collections::BTreeMap, vec::Vec};
use thiserror::Error;

pub type Mid = u64;
pub type MessageChannelId = u64;
pub type Result<T> = core::result::Result<T, MessageError>;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Invalid Mid")]
    InvalidMid,
}

#[derive(Default, Clone)]
pub struct MessageStore {
    next_mid: Mid,
    data: BTreeMap<Mid, Message>,
}

#[derive(Default, Clone)]
pub struct Message {
    pub waited_on: bool,
    data: Vec<u8>,
    read_pointer: usize,
}

impl Message {
    pub fn new(data: Vec<u8>) -> Self {
        Message {
            waited_on: false,
            data,
            read_pointer: 0,
        }
    }
    fn remaining(&self) -> usize {
        self.data.len() - self.read_pointer
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    /// reads AT MOST amt bytes
    fn read_bytes(&mut self, amt: usize) -> (Vec<u8>, bool) {
        let is_empty;
        let to_read = if amt < self.remaining() {
            is_empty = false;
            amt
        } else {
            is_empty = true;
            self.remaining()
        };
        let mut allocation = Vec::with_capacity(to_read);

        (self.read_pointer..(self.read_pointer + to_read)).for_each(|i| {
            // SAFETY should be in bounds due to previous bounds related code
            allocation.push(self.data.as_slice()[i]);
        });
        self.read_pointer += to_read;

        (allocation, is_empty)
    }
}

impl MessageStore {
    pub fn get_unwaited_message(&self) -> Option<Mid> {
        self.data
            .iter()
            .find(|item| !item.1.waited_on)
            .map(|some| some.0.clone())
    }
    pub fn new() -> Self {
        MessageStore::default()
    }
    pub fn push_message(&mut self, message: Message) -> Mid {
        let mid = self.next_mid;
        self.data.insert(mid, message);
        self.next_mid += 1;
        return mid;
    }
    pub fn get_message_mut(&mut self, mid: &Mid) -> Result<&mut Message> {
        self.data.get_mut(mid).ok_or(MessageError::InvalidMid)
    }
    /// returns either the length of the message, or the requested length (whichever is less)
    pub fn read_message(&mut self, mid: Mid, len: usize) -> Result<Vec<u8>> {
        let Result::Ok(message) = self.get_message_mut(&mid) else {
            return Err(MessageError::InvalidMid);
        };

        let (allocation, is_empty) = message.read_bytes(len);
        if is_empty {
            self.data
                .remove(&mid)
                .expect("should exist as per previous check");
        }

        Result::Ok(allocation)
    }
}
