use crate::scheduler::process::threads::Tid;

#[derive(Default, Clone, Debug)]
pub enum WaitState {
    #[default]
    None,
    TidWait(Tid),
}

pub enum WaitStateUpdate {
    ThreadFinished(Tid),
}

impl WaitState {
    pub fn is_none(&self) -> bool {
        match self {
            WaitState::None => true,
            _ => false,
        }
    }
    pub fn update(&mut self, update: &WaitStateUpdate) -> bool {
        match update {
            WaitStateUpdate::ThreadFinished(update_tid) => {
                if let WaitState::TidWait(tid) = self {
                    if update_tid == tid {
                        *self = WaitState::None;
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
