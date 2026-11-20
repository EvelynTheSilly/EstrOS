use crate::VFSPersistant;
use crate::packet_handlers::ErrorCode;
use std::os::estros::cap::Capability;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[repr(u8)]
pub enum OpenHandlerError {
    #[error("No such file or directory")]
    NotFound = 1,
}

impl ErrorCode for OpenHandlerError {
    fn code(self) -> u8 {
        self as u8
    }
}

pub fn open_handler(
    vfs: &mut VFSPersistant,
    data: Vec<u8>,
    _caps: Vec<Capability>,
) -> Result<(Vec<u8>, Vec<Capability>), OpenHandlerError> {
    if let Some(cap) = vfs.fs.get(&data) {
        return Ok((Vec::new(), vec![cap.duplicate()]));
    };
    return Err(OpenHandlerError::NotFound);
}
