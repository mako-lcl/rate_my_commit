use serde::{Deserialize, Serialize};

use crate::Rating;

#[derive(Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<CompletionMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompletionMessageResponse {
    pub content: Rating,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompletionChoice {
    pub message: CompletionMessage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    pub choices: Vec<CompletionChoice>,
}
