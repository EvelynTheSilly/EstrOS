use crate::scheduler::process::threads::Tid;

#[derive(Default, Clone)]
pub enum WaitState {
    #[default]
    None,
    TidWait(Tid),
}

impl WaitState {
    pub fn is_none(&self) -> bool {
        match self {
            WaitState::None => true,
            _ => false,
        }
    }
}
