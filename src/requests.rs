use serde::{Deserialize, Serialize};

use crate::models::command::Command;

pub mod auth;
pub mod message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T>
where
    T: RequestBody,
{
    pub command: Command,
    pub body: T,
    pub token: String,
}

impl<T: RequestBody> Request<T> {
    pub fn new(command: Command, body: T, token: String) -> Self {
        Self {
            command,
            body,
            token,
        }
    }
}

pub trait RequestBody {
    fn op(&self) -> Command;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyRequestBody {}

impl RequestBody for EmptyRequestBody {
    fn op(&self) -> Command {
        todo!();
    }
}
