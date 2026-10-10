use crate::scheduler::process::{Pid, messages::MessageRecieverChannelId};

#[derive(Clone)]
pub struct ProcessMessageCapability {
    pub proc: Pid,
    pub reciever_id: MessageRecieverChannelId,
}
