use crate::{VFSPersistant, packet_handlers::ErrorCode};
use std::os::estros::cap::Capability;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[repr(u8)]
pub enum CreateHandlerError {
    #[error("Invalid character in the path")]
    InvalidCharacter = 1,
}

impl ErrorCode for CreateHandlerError {
    fn code(self) -> u8 {
        self as u8
    }
}

pub fn create_handler(
    vfs: &mut VFSPersistant,
    data: Vec<u8>,
    mut caps: Vec<Capability>,
) -> Result<(Vec<u8>, Vec<Capability>), CreateHandlerError> {
    if data.contains(&0u8) {
        return Err(CreateHandlerError::InvalidCharacter);
    };
    if data.contains(&10u8) {
        return Err(CreateHandlerError::InvalidCharacter);
    };
    vfs.fs.insert(data, caps.remove(0));
    return Ok((Vec::new(), Vec::new()));
}
