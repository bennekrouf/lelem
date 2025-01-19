use serde::{Deserialize, Serialize};

// Structure for the Ollama generate request
#[derive(Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub format: Option<String>,
}

// Structure for Ollama's response chunks
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    // done: bool,
}
