use std::{
    collections::HashMap,
    error::Error,
    os::estros::{
        cap::{Capability, message::Message},
        syscalls,
    },
};
use thiserror::Error;
use vfs_proto::VFSPacketHeader;

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
            .get(0)
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
        println!("header data: {:?}", header);
        let data = {
            let mut vec = Vec::new();
            let mut data = [0u8; 64];
            while let Some(read) = msg.read_bytes(&mut data) {
                println!("vfs: read chunk {}", read);
                vec.extend_from_slice(&data[..read]);
                if read == 0 {
                    println!("vfs: zero read, breaking");
                    break;
                }
            }
            vec
        };
        println!("vfs: processed body, {} bytes, {} caps", data.len(), caps.len());
        match header.packet_type {
            // create:
            // data will be used for a utf8 string path restricted to
            1 => 'handler: {
                if data.contains(&0u8) {
                    break 'handler;
                };
                if data.contains(&10u8) {
                    break 'handler;
                };
                self.fs.insert(data, caps.remove(1));
            }
            2 => 'handler: {
                println!("vfs: lookup for {:?}", data);
                if let Some(cap) = self.fs.get(&data) {
                    responce.send_message(&[], vec![cap.duplicate()]);
                    println!("vfs: lookup answered");
                } else {
                    println!("vfs: lookup MISS");
                };
            }
            _ => {}
        };
        Ok(())
    }
    pub fn serve(&mut self) -> ! {
        loop {
            println!("waiting for message");
            let msg = self
                .root_reciever_cap
                .wait_message()
                .expect("failed to wait on capability");
            println!("got message");
            let res = self.process_message(msg);
            if let Err(err) = res {
                println!("ERROR: {}", err)
            }
        }
    }
}

fn main() {
    assert!(size_of::<VFSPacketHeader>() == 16);
    println!("starting vfs driver");
    let responce = unsafe { Capability::from_id(0) };
    let cap = Capability::spawn_message();
    responce.send_message(&[], vec![cap.duplicate()]);

    VFSPersistant::new(cap).serve()
}
