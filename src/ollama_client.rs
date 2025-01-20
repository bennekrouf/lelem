use serde::{Deserialize, Serialize};
use reqwest;
use std::error::Error;

// Structure for the Ollama generate request
#[derive(Serialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub format: Option<String>,
}

// Structure for Ollama's response chunks
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

/// Client for interacting with Ollama API
pub struct OllamaClient {
    client: reqwest::Client,
}

impl OllamaClient {
    /// Create a new Ollama client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Generate a JSON response from Ollama
    pub async fn generate_json(&self, model: &str, prompt: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let request_body = OllamaGenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            format: Some("json".to_string()),
        };

        // Perform the HTTP request
        let response = self.client.post("http://localhost:11434/api/generate")
            .json(&request_body)
            .send()
            .await?;

        // Collect the full response text
        let response_obj = response.json::<OllamaResponse>().await?;
        let full_response_text = response_obj.response.trim().to_owned();

        // Attempt to parse the JSON
        let parsed_json: serde_json::Value = serde_json::from_str(&full_response_text)
            .map_err(|_| format!("Failed to parse JSON. Raw response: {}", full_response_text))?;

        Ok(parsed_json)
    }
}
