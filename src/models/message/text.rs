use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{message::MessageContent, msg_type::MessageType};

#[derive(Debug, Serialize, Deserialize)]
pub struct TextMessage {
    pub text: String,
}

impl TextMessage {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl MessageContent for TextMessage {
    fn get_type(&self) -> Option<MessageType> {
        Some(MessageType::Text)
    }

    fn get_text(&self) -> Option<String> {
        Some(self.text.to_owned())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotStringError;

impl FromStr for TextMessage {
    type Err = NotStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text_message: Self = TextMessage {
            text: s.to_owned(),
        };
        match text_message {
            text => Ok(text),
        }
    }
}
