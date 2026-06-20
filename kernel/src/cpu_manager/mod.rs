//! keeps track of the state of all cpu cores

use alloc::collections::btree_map::BTreeMap;

use crate::syncronisation::GlobalSharedLock;

/// a set of data the persists when a cpu
pub struct CpuPersistantState {
    previous_ttbr: Option<usize>,
    executing_pid: Option<u64>,
    executing_tid: Option<u64>,
}

impl CpuPersistantState {
    pub const fn new() -> Self {
        Self {
            previous_ttbr: None,
            executing_pid: None,
            executing_tid: None,
        }
    }
    pub fn submit_pid_tid(&mut self, pid: u64, tid: u64) {
        self.executing_pid = Some(pid);
        self.executing_tid = Some(tid);
    }
    pub fn get_pid(&mut self) -> Option<u64> {
        let pid = self.executing_pid;
        self.executing_pid = None;
        pid
    }
    pub fn get_tid(&mut self) -> Option<u64> {
        let tid = self.executing_tid;
        self.executing_tid = None;
        tid
    }
    pub fn submit_ttbr(&mut self, ttbr: usize) {
        self.previous_ttbr = Some(ttbr)
    }
    pub fn get_ttbr(&mut self) -> Option<usize> {
        let ttbr = self.previous_ttbr;
        self.previous_ttbr = None;
        ttbr
    }
}

pub fn get_cpu_id() -> u64 {
    // TODO: make it actually get cpu id's
    0
}

pub static CPU_STATE_MANAGER: GlobalSharedLock<BTreeMap<u64, CpuPersistantState>> =
    GlobalSharedLock::new(BTreeMap::<u64, CpuPersistantState>::new());
