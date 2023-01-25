use core::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum MessageType {
    Text,
    Audio,
    Video,
    File,
}

impl MessageType {
    pub fn get_index(&self) -> u8 {
        serde_json::to_string(self).unwrap().parse::<u8>().unwrap()
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
