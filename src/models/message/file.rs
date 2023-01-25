use serde::{Deserialize, Serialize};

use super::{message::MessageContent, msg_type::MessageType};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMessage {
    pub text: String,
    pub filename: String,
    pub file: Vec<u8>,
}

impl FileMessage {
    pub fn new(text: &str, filename: &str, file: Vec<u8>) -> Self {
        Self {
            text: text.to_owned(),
            filename: filename.to_owned(),
            file: file,
        }
    }
}

impl MessageContent for FileMessage {
    fn get_type(&self) -> Option<MessageType> {
        Some(MessageType::File)
    }

    fn get_text(&self) -> Option<String> {
        Some(self.text.to_owned())
    }
}
