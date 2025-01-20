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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Endpoint {
    pub id: String,
    pub text: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    required: bool,
    alternatives: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub endpoints: Vec<Endpoint>,
}
