use flume::{Sender};

pub struct FileChannelMessage {
    pub name: String,
    pub bytes: Vec<u8>,
    pub r#type: String
}

#[derive(Clone)]
pub struct AppState {
    pub files_sender: Sender<FileChannelMessage>,
    pub redis: std::sync::Arc<tokio::sync::Mutex<crate::redis_service::RedisService>>,
}