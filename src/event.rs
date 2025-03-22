use crate::server::ServerStatus;

pub enum ServerEvent {
    PingStatus { id: usize, status: ServerStatus },
    RefreshRequest { id: usize },
    RemoveServer { id: usize },
}
