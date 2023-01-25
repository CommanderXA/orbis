use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSides {
    sender: Uuid,
    receiver: Uuid,
}

impl MessageSides {
    pub fn new(sender: Uuid, receiver: Uuid) -> Self {
        Self { sender, receiver }
    }

    pub fn get_sender(&self) -> Uuid {
        self.sender
    }

    pub fn get_receiver(&self) -> Uuid {
        self.receiver
    }
}
