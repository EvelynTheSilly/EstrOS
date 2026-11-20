use crate::packet_handlers::{
    create::create_handler, error_to_code_plus_error, open::open_handler,
};
use std::{
    collections::HashMap,
    error::Error,
    os::estros::cap::{Capability, message::Message},
};
use thiserror::Error;
use vfs_proto::{
    VFSPacketHeader,
    packets::{CREATE, OPEN},
};

mod packet_handlers;

struct VFSPersistant {
    root_reciever_cap: Capability,
    /// path to cap
    fs: HashMap<Vec<u8>, Capability>,
}

#[derive(Debug, Error)]
enum MessageProcessingError {
    #[error("no responce cap")]
    ResponceCapError,
    #[error("invalid header")]
    HeaderError,
    #[error("invalid packet type")]
    PacketTypeError,
}

impl VFSPersistant {
    pub fn new(cap_to_serve_on: Capability) -> Self {
        VFSPersistant {
            root_reciever_cap: cap_to_serve_on,
            fs: HashMap::default(),
        }
    }
    fn process_message(&mut self, msg: Message) -> Result<(), Box<dyn Error>> {
        let mut caps = msg.read_caps();
        if caps.len() == 0 {
            println!("got the length {}", caps.len());
            return Err(Box::new(MessageProcessingError::ResponceCapError));
        }
        let responce = caps
            .try_remove(0)
            .ok_or(Box::new(MessageProcessingError::ResponceCapError))?;
        let mut data = [0u8; 16];
        let bytes_read = msg
            .read_bytes(&mut data)
            .ok_or(Box::new(MessageProcessingError::HeaderError))?;
        if bytes_read != 16 {
            println!("read {} sized header", bytes_read);
            return Err(Box::new(MessageProcessingError::HeaderError));
        }
        let header: VFSPacketHeader = unsafe { std::ptr::read(data.as_ptr() as *const _) };
        let data = {
            let mut vec = Vec::new();
            let mut data = [0u8; 64];
            while let Some(read) = msg.read_bytes(&mut data) {
                vec.extend_from_slice(&data[..read]);
                if read == 0 {
                    break;
                }
            }
            vec
        };
        let result: Result<(Vec<u8>, Vec<Capability>), (u8, Box<dyn Error>)> =
            match header.packet_type {
                CREATE => error_to_code_plus_error(create_handler(self, data, caps)),
                OPEN => error_to_code_plus_error(open_handler(self, data, caps)),
                _ => Err((u8::MAX, Box::new(MessageProcessingError::PacketTypeError))),
            };
        match result {
            Ok((data, caps)) => {
                let mut ret_data = vec![0];
                ret_data.extend_from_slice(data.as_slice());
                responce.send_message(&ret_data.as_slice(), caps);
            }
            Err((code, error)) => {
                let mut ret_data = vec![code];
                ret_data.extend_from_slice(&(*error.to_string().as_bytes()));
                responce.send_message(&ret_data.as_slice(), Vec::new());
            }
        }
        Ok(())
    }
    pub fn serve(&mut self) -> ! {
        loop {
            let msg = self
                .root_reciever_cap
                .wait_message()
                .expect("failed to wait on capability");
            let res = self.process_message(msg);
            if let Err(err) = res {
                println!("ERROR: {}", err)
            }
        }
    }
}

fn main() {
    assert!(size_of::<VFSPacketHeader>() == 16);
    let responce = unsafe { Capability::from_id(0) };
    let cap = Capability::spawn_message();
    responce.send_message(&[], vec![cap.duplicate()]);

    VFSPersistant::new(cap).serve()
}
