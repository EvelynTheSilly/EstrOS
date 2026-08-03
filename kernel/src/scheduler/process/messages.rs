use alloc::{collections::BTreeMap, vec::Vec};
use thiserror::Error;


pub type Mid = u64;
pub type Result<T> = core::result::Result<T, MessageError>;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Invalid Mid")]
    InvalidMid,
}

#[derive(Default)]
pub struct MessageStore {
    next_mid: Mid,
    data: BTreeMap<Mid, Message>,
}

pub struct Message {
    data: Vec<u8>,
    read_pointer: usize,
}

impl Message {
    fn new(data: Vec<u8>) -> Self {
        Message {
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
    pub fn new() -> Self {
        let mut data = BTreeMap::new();
        data.insert(
            0 as Mid,
            Message::new("this is a message which is being read".as_bytes().to_vec()),
        );
        MessageStore { next_mid: 0, data }
    }
    pub fn push_message(&mut self, message: Message) {
        self.data.insert(self.next_mid, message);
        self.next_mid += 1;
    }
    fn get_message_mut(&mut self, mid: &Mid) -> Result<&mut Message> {
        self.data.get_mut(mid).ok_or(MessageError::InvalidMid)
    }
    /// returns either the length of the message, or the requested length (whichever is less)
    pub fn read_message(&mut self, mid: Mid, len: usize) -> Result<Vec<u8>> {
        let Result::Ok(message) = self.get_message_mut(&mid) else {
            panic!();
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
