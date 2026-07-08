mod qds;
mod round_robin;

pub type GlobalScheduler = round_robin::RoundRobinScheduler;
//pub type GlobalScheduler = qds::QDScheduler;
