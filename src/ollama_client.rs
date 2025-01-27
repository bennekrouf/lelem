use serde::{Deserialize, Serialize};
use reqwest;
use std::error::Error;

const DEFAULT_MODEL: &str = "deepseek-r1:8b";

#[derive(Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

pub struct OllamaClient {
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Generate a JSON response from Ollama
    pub async fn generate_json(&self, prompt: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let full_response_text = call_ollama(prompt).await?;

        let parsed_json: serde_json::Value = serde_json::from_str(&full_response_text)
            .map_err(|_| format!("Failed to parse JSON. Raw response: {}", full_response_text))?;

        Ok(parsed_json)
    }
}

// Base Ollama call function
pub async fn call_ollama(prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        model: DEFAULT_MODEL.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        format: Some("json".to_string()),
    };

    let response = client.post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?;

    let response_obj = response.json::<OllamaResponse>().await?;
    Ok(response_obj.response.trim().to_owned())
}
