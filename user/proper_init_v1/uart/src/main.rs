use std::os::estros::cap::Capability;
use std::os::estros::syscalls;
use vfs_proto::VFSPacketHeader;

const PACKET_TYPE_CREATE: u8 = 1;
const DEVICE_PATH: &[u8] = b"/dev/uart";

const UART_VADDR: usize = 0x670000;
const UART_FR_OFFSET: usize = 0x18;
const UART_FR_TXFF: u8 = 1 << 5;

const RX_BUF_LEN: usize = 64;

fn uart_write(uart: *mut u8, byte: u8) {
    unsafe {
        while core::ptr::read_volatile(uart.add(UART_FR_OFFSET)) & UART_FR_TXFF != 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(uart, byte);
    }
}

fn main() {
    println!("uart driver starting");

    // caps handed over by entry, in spawn order:
    // 0 = vfs cap, 1 = entry ack cap, 2 = uart device mem cap
    let vfs_cap = unsafe { Capability::from_id(0) };
    let ack_cap = unsafe { Capability::from_id(1) };
    let dev_cap = unsafe { Capability::from_id(2) };

    unsafe { syscalls::map_device_memory(UART_VADDR, dev_cap.id()) };
    let uart = UART_VADDR as *mut u8;

    let serving_cap = Capability::spawn_message();

    let dummy_resp = Capability::spawn_message();
    let header = VFSPacketHeader::new(PACKET_TYPE_CREATE, 0);
    let mut data = header.as_u8_slice();
    data.extend_from_slice(DEVICE_PATH);
    vfs_cap.send_message(&data, vec![dummy_resp, serving_cap.duplicate()]);

    ack_cap.send_message(&[], vec![]);
    println!("uart driver ready");

    loop {
        let msg = serving_cap
            .wait_message()
            .expect("uart driver: failed to wait on serving cap");
        println!("uart driver: got write request");
        let mut buf = [0u8; RX_BUF_LEN];
        while let Some(read) = msg.read_bytes(&mut buf) {
            if read == 0 {
                break;
            }
            for byte in &buf[..read] {
                uart_write(uart, *byte);
            }
            if read < buf.len() {
                break;
            }
        }
        println!("uart driver: wrote {}+ bytes to uart", RX_BUF_LEN);
    }
}
