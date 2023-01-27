use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T>
where
    T: ResponseBody,
{
    pub status: ResponseStatus,
    pub content: T,
}

impl<T: ResponseBody> Response<T> {
    pub fn new(status: ResponseStatus, content: T) -> Self {
        Self { status, content }
    }
}

pub trait ResponseBody {}

impl ResponseBody for String {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    Ok,
    Err,
}
