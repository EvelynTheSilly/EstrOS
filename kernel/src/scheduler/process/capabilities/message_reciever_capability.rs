use crate::scheduler::process::{Pid, messages::MessageRecieverChannelId};

#[derive(Clone, Debug)]
pub struct ProcessMessageCapability {
    pub proc: Pid,
    pub reciever_id: MessageRecieverChannelId,
}
