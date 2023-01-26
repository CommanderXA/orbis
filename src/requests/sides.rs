use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSides {
    sender: Uuid,
    receiver: Uuid,
}

impl RequestSides {
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
