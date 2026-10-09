use crate::scheduler::process::{Pid, messages::MessageRecieverChannelId};

pub struct ProcessMessageCapability {
    pub proc: Pid,
    pub reciever_id: MessageRecieverChannelId,
}
