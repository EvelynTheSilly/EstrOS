use core::mem::MaybeUninit;

use alloc::string::String;

use crate::{
    println,
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    vectors::cpu_state::State,
};

pub fn write_to_uart(state: &mut State, pid: u64) {
    PROCESS_MANAGER.lock(|scheduler| {
        let mut s;
        let dest: &mut [u8];
        unsafe {
            s = String::with_capacity(state.x[1] as usize);
            let spare: &mut [MaybeUninit<u8>] = s.as_mut_vec().spare_capacity_mut();

            // SAFETY: ptr is non-null, properly aligned, and the function will fully initialize all len bytes
            dest = core::slice::from_raw_parts_mut(spare.as_mut_ptr() as *mut u8, spare.len());
        };
        scheduler
            .process_mem_read(pid, dest, state.x[0] as usize)
            .expect("failed to read process memory TODO: HANDLE THIS");
        unsafe {
            let slen = s.len();
            s.as_mut_vec().set_len(slen + dest.len());
        }
        println!("{}", s);
    })
}
