use std::os::estros::cap;
use std::os::estros::syscalls;

static UART_ELF: &[u8] = include_bytes!(env!("UART_ELF"));
static UART_USER_ELF: &[u8] = include_bytes!(env!("UART_USER_ELF"));
static VFS_ELF: &[u8] = include_bytes!(env!("VFS_ELF"));

/// uart device on qemu virt, 1 page
const UART_MMIO_ADDR: usize = 0x900_0000;
const UART_MMIO_SIZE: usize = 4096;

fn main() {
    println!("\n\nstarting estros userspace");
    println!("spawning vfs");
    let response = cap::Capability::spawn_message();
    let spawn_caps = [response.duplicate()];

    unsafe {
        syscalls::spawn_process(
            VFS_ELF.as_ptr(),
            VFS_ELF.len(),
            spawn_caps.as_ptr() as *const u8,
            spawn_caps.len(),
        );
    };
    println!("entry: waiting for vfs driver");
    let message = response.wait_message().unwrap();
    println!("recieved message");
    let vfs_cap = message.read_caps().remove(0);

    println!("entry: spawning uart driver");
    // uart driver receives, in order (from_id 0,1,2):
    // 0 = vfs cap, 1 = ack cap, 2 = uart device cap
    let ack_cap = cap::Capability::spawn_message();
    let uart_caps = [
        vfs_cap.duplicate(),
        ack_cap.duplicate(),
        cap::Capability::from_mem(UART_MMIO_ADDR, UART_MMIO_SIZE).unwrap(),
    ];
    unsafe {
        syscalls::spawn_process(
            UART_ELF.as_ptr(),
            UART_ELF.len(),
            uart_caps.as_ptr() as *const u8,
            uart_caps.len(),
        );
    };
    ack_cap.wait_message().unwrap();
    println!("entry: uart driver registered /dev/uart");

    println!("entry: spawning uart_user");
    let user_caps = [vfs_cap.duplicate()];
    unsafe {
        syscalls::spawn_process(
            UART_USER_ELF.as_ptr(),
            UART_USER_ELF.len(),
            user_caps.as_ptr() as *const u8,
            user_caps.len(),
        );
    };
}
