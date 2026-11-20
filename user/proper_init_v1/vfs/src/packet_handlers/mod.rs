use std::{error::Error, os::estros::cap::Capability};

pub mod create;
pub mod open;

trait ErrorCode {
    fn code(self) -> u8;
}

impl ErrorCode for ! {
    fn code(self) -> u8 {
        match self {}
    }
}

#[allow(private_bounds)]
pub fn error_to_code_plus_error<E: ErrorCode + Error + Clone + 'static>(
    result: Result<(Vec<u8>, Vec<Capability>), E>,
) -> Result<(Vec<u8>, Vec<Capability>), (u8, Box<dyn Error>)> {
    result.map_err(|err| (err.clone().code(), err.into()))
}
