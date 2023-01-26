use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{crypto::aes::vec_to_string, requests::sides::RequestSides};

use super::{msg_type::MessageType, status::MessageStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T>
where
    T: MessageContent,
{
    pub uuid: Uuid,
    pub content: T,
    nonce: String,
    pub sides: RequestSides,
    pub status: MessageStatus,
    pub ttl: Option<i64>,
    pub secret: bool,

    msg_type: MessageType,
    created_at: i64,
}

impl<T: MessageContent> Message<T> {
    pub fn new(content: T, nonce: Vec<u8>, sender: Uuid, receiver: Uuid) -> Self {
        let msg_type: MessageType = content.get_type().unwrap();
        Self {
            uuid: Uuid::new_v4(),
            content: content,
            nonce: vec_to_string(nonce),
            sides: RequestSides::new(sender, receiver),
            status: MessageStatus::new(),
            ttl: None,
            secret: false,
            msg_type: msg_type,
            created_at: Utc::now().timestamp(),
        }
    }

    pub fn get_nonce(&self) -> String {
        self.nonce.to_owned()
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at, 0).unwrap()
    }

    pub fn get_msg_type(&self) -> MessageType {
        self.msg_type
    }
}

pub trait MessageContent {
    fn get_type(&self) -> Option<MessageType>;

    fn get_text(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyMessageBody {}

impl MessageContent for EmptyMessageBody {
    fn get_type(&self) -> Option<MessageType> {
        None
    }

    fn get_text(&self) -> Option<String> {
        None
    }
}
