#![allow(dead_code)]

use crate::{mem::paging::kernel_virtual_to_physical, syncronisation::GlobalSharedLock};
use alloc::alloc::{Layout, alloc_zeroed};
use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

static UART_BASE: AtomicU64 = AtomicU64::new(0xFFFF_0000_0900_0000);

const UART_PHYS: u64 = 0x0900_0000;
const UART_MMIO_VA: u64 = 0xFFFF_0000_0900_0000;

#[inline]
fn uart_data_register() -> *mut u8 {
    UART_BASE.load(Ordering::Relaxed) as *mut u8
}
#[inline]
fn uart_flag_register() -> *mut u8 {
    (UART_BASE.load(Ordering::Relaxed) + 0x18) as *mut u8
}
#[inline]
fn uart_interrupt_mask_register() -> *mut u8 {
    (UART_BASE.load(Ordering::Relaxed) + 0x38) as *mut u8
}
#[inline]
fn uart_interrupt_clear_register() -> *mut u8 {
    (UART_BASE.load(Ordering::Relaxed) + 0x44) as *mut u8
}

unsafe fn hang() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn map_uart_mmio(hhdm_base: u64) {
    unsafe {
        let mut ttbr: u64;
        asm!("mrs {r}, ttbr1_el1", r = out(reg) ttbr);
        let mut table_va = hhdm_base + (ttbr & 0x0000_FFFF_FFFF_F000);
        for shift in [39u32, 30, 21, 12] {
            let index = ((UART_MMIO_VA >> shift) & 0x1FF) as usize;
            let entry_ptr = (table_va + index as u64 * 8) as *mut u64;
            let entry = entry_ptr.read_volatile();
            if shift == 12 {
                entry_ptr.write_volatile(UART_PHYS | 0b1 | (0b1 << 1) | (0b10 << 2) | (0b1 << 10) | (0b1 << 11));
                return;
            }
            let desc_type = entry & 0b11;
            if desc_type == 0b11 {
                table_va = hhdm_base + (entry & 0x0000_FFFF_FFFF_F000);
            } else if desc_type == 0b01 {
                hang();
            } else {
                let layout = Layout::from_size_align(4096, 4096).unwrap();
                let new_table = alloc_zeroed(layout) as *mut u64;
                if new_table.is_null() {
                    hang();
                }
                let child_phys = kernel_virtual_to_physical(new_table as *mut u8) as u64;
                entry_ptr.write_volatile(child_phys | 0b11);
                table_va = hhdm_base + child_phys;
            }
        }
    }
}

pub struct Uart;

pub static UART: GlobalSharedLock<Uart> = GlobalSharedLock::new(Uart);

pub fn init(hhdm_base: u64) {
    map_uart_mmio(hhdm_base);
    UART_BASE.store(hhdm_base + 0x0900_0000, Ordering::Relaxed);
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            unsafe {
                core::ptr::write_volatile(uart_data_register(), byte);
            }
        }
        Ok(())
    }
}

pub unsafe fn write_string(string: &str) {
    for &byte in string.as_bytes() {
        unsafe {
            core::ptr::write_volatile(uart_data_register(), byte);
        }
    }
}

pub unsafe fn write_byte(char: u8) {
    unsafe {
        core::ptr::write_volatile(uart_data_register(), char);
    }
}

#[macro_export]
macro_rules! println {
    () => {
        let _ = $crate::print!("\n\r");
        ()
    };
    ($($arg:tt)*) => {{
        use $crate::uart::Uart;
        use $crate::uart::UART;
        use $crate::syncronisation::Mutex;
        use core::fmt::Write;
        UART.lock(|mut uart|{
            let _ = Uart::write_fmt(&mut uart,core::format_args!($($arg)*));
            let _ = Uart::write_fmt(&mut uart,core::format_args!("\n\r"));
        });
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::uart::Uart;
        use $crate::uart::UART;
        use $crate::syncronisation::Mutex;
        use core::fmt::Write;
        UART.lock(|mut uart|{
            let _ = Uart::write_fmt(&mut uart,core::format_args!($($arg)*));
        })
    }};
}

#[allow(unused)]
pub(crate) use {print, println};
