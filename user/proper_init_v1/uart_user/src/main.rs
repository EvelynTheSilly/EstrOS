use std::os::estros::cap::Capability;
use vfs_proto::VFSPacketHeader;

const PACKET_TYPE_LOOKUP: u8 = 2;
const DEVICE_PATH: &[u8] = b"/dev/uart";

fn main() {
    println!("uart_user starting");

    // cap 0 = vfs cap handed over by entry
    let vfs_cap = unsafe { Capability::from_id(0) };
    println!("uart_user: got vfs cap");

    // lookup /dev/uart in the vfs
    let resp_cap = Capability::spawn_message();
    let header = VFSPacketHeader::new(PACKET_TYPE_LOOKUP, 0);
    let mut data = header.as_u8_slice();
    data.extend_from_slice(DEVICE_PATH);
    vfs_cap.send_message(&data, vec![resp_cap.duplicate()]);
    println!("uart_user: lookup sent");

    let msg = resp_cap
        .wait_message()
        .expect("uart_user: no reply from vfs");
    println!("uart_user: got reply");
    let caps = msg.read_caps();
    let uart_cap = caps
        .first()
        .expect("uart_user: vfs reply contained no capability");
    println!("uart_user: got uart driver cap");

    uart_cap.send_message(b"hello from uart_user via vfs\n", vec![]);
    println!("uart_user sent message through uart driver");
}
