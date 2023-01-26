use serde::{Deserialize, Serialize};

use crate::models::calls::audio_call::AudioCall;

use super::Command;
use super::RequestBody;

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioCallRequest {
    pub call: AudioCall,
}

impl AudioCallRequest {
    pub fn new(audio_call: AudioCall) -> Self {
        Self { call: audio_call }
    }
}

impl RequestBody for AudioCallRequest {
    fn op(&self) -> Command {
        Command::AudioCall
    }
}
