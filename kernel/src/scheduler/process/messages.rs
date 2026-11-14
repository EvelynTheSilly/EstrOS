use crate::{println, scheduler::process::capabilities::Capability};
use alloc::{collections::BTreeMap, vec::Vec};
use thiserror::Error;

pub type Mid = u64;
pub type MessageRecieverChannelId = u64;
pub type Result<T> = core::result::Result<T, MessageError>;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Invalid Mid")]
    InvalidMid,
}

#[derive(Default, Clone)]
pub struct MessageRecieverStore {
    rolling_id: MessageRecieverChannelId,
    pub message_channels: BTreeMap<MessageRecieverChannelId, MessageStore>,
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
    caps: Vec<Capability>,
    read_pointer: usize,
}

impl MessageRecieverStore {
    fn new_id(&mut self) -> MessageRecieverChannelId {
        let id = self.rolling_id;
        self.rolling_id += 1;
        id
    }
    pub fn insert_reciever(&mut self, store: MessageStore) -> MessageRecieverChannelId {
        let id = self.new_id();
        self.message_channels.insert(id, store);
        id
    }
}

impl Message {
    pub fn new(data: Vec<u8>, caps: Vec<Capability>) -> Self {
        Message {
            waited_on: false,
            data,
            caps,
            read_pointer: 0,
        }
    }
    fn is_empty(&self) -> bool {
        self.caps.is_empty() && self.data.len() == self.read_pointer
    }
    fn remaining_data(&self) -> usize {
        self.data.len() - self.read_pointer
    }
    fn remaining_caps(&self) -> usize {
        self.caps.len()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    /// reads AT MOST amt bytes
    fn read_bytes(&mut self, amt: usize) -> Vec<u8> {
        let to_read = if amt < self.remaining_data() {
            amt
        } else {
            self.remaining_data()
        };
        let mut allocation = Vec::with_capacity(to_read);

        (self.read_pointer..(self.read_pointer + to_read)).for_each(|i| {
            // SAFETY should be in bounds due to previous bounds related code
            allocation.push(self.data.as_slice()[i]);
        });
        self.read_pointer += to_read;

        allocation
    }
    /// reads AT MOST amt caps
    fn read_caps(&mut self, amt: usize) -> Vec<Capability> {
        let to_read = if amt < self.remaining_caps() {
            amt
        } else {
            self.remaining_caps()
        };
        let first = Vec::from(&self.caps.as_slice()[0..to_read]);
        let last = Vec::from(&self.caps.as_slice()[to_read..]);
        self.caps = last;

        first
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
    pub fn read_message_data(&mut self, mid: Mid, len: usize) -> Result<Vec<u8>> {
        let Result::Ok(message) = self.get_message_mut(&mid) else {
            return Err(MessageError::InvalidMid);
        };

        let allocation = message.read_bytes(len);
        if message.is_empty() {
            self.data
                .remove(&mid)
                .expect("should exist as per previous check");
        }

        Result::Ok(allocation)
    }
    /// returns either the length of the message, or the requested length (whichever is less)
    pub fn read_message_caps(&mut self, mid: Mid, len: usize) -> Result<Vec<Capability>> {
        let Result::Ok(message) = self.get_message_mut(&mid) else {
            println!("invalid mid {}", mid);
            return Err(MessageError::InvalidMid);
        };

        let allocation = message.read_caps(len);
        if message.is_empty() {
            self.data
                .remove(&mid)
                .expect("should exist as per previous check");
        }

        Result::Ok(allocation)
    }
}
