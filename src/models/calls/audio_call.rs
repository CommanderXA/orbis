use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::requests::sides::RequestSides;

use super::call_type::CallType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioCall {
    pub uuid: Uuid,
    pub sides: RequestSides,
    pub secret: bool,

    created_at: i64,
}

impl AudioCall {
    pub fn new(sender: Uuid, receiver: Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            sides: RequestSides::new(sender, receiver),
            secret: false,
            created_at: Utc::now().timestamp(),
        }
    }

    pub fn get_type(&self) -> CallType {
        CallType::Audio
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at, 0).unwrap()
    }
}
