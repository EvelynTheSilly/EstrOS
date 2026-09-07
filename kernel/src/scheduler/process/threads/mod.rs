use crate::vectors::cpu_state::State;
use alloc::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ThreadError {
    #[error("Invalid Tid")]
    InvalidTid,
}
type Result<T> = core::result::Result<T, ThreadError>;
type Tid = u64;

#[derive(Clone)]
pub struct ThreadStore {
    running_tid: Tid,
    threads: BTreeMap<Tid, SchedulerThread>,
}

#[derive(Clone, Default)]
pub struct SchedulerThread {
    pub state: State,
}
impl SchedulerThread {
    pub fn at(location: u64) -> Self {
        SchedulerThread {
            state: State {
                elr: location,
                ..State::default()
            },
        }
    }
}

impl ThreadStore {
    pub fn new() -> Self {
        ThreadStore {
            running_tid: 0,
            threads: BTreeMap::new(),
        }
    }
    fn next_tid(&mut self) -> Tid {
        self.running_tid = self.running_tid + 1;
        return self.running_tid;
    }
    pub fn iter(&self) -> alloc::collections::btree_map::Iter<'_, u64, SchedulerThread> {
        self.threads.iter()
    }
    pub fn iter_mut(&mut self) -> alloc::collections::btree_map::IterMut<'_, u64, SchedulerThread> {
        self.threads.iter_mut()
    }
    pub fn get(&self, tid: Tid) -> Result<&SchedulerThread> {
        self.threads.get(&tid).ok_or(ThreadError::InvalidTid)
    }
    pub fn get_mut(&mut self, tid: Tid) -> Result<&mut SchedulerThread> {
        self.threads.get_mut(&tid).ok_or(ThreadError::InvalidTid)
    }
    pub fn remove(&mut self, tid: Tid) -> Result<()> {
        self.threads
            .remove(&tid)
            .map(|_| ())
            .ok_or(ThreadError::InvalidTid)
    }
    pub fn spawn(&mut self, thread: SchedulerThread) -> Tid {
        let tid = self.next_tid();
        self.threads.insert(tid, thread);
        tid
    }
    pub fn report_thread_state(&mut self, tid: u64, state: State) -> Result<()> {
        self.threads
            .get_mut(&tid)
            .ok_or(ThreadError::InvalidTid)?
            .state = state;
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.threads.len()
    }
}
