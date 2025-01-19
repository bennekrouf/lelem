use crate::models::GenerateRequest;
use crate::models::OllamaResponse;

use std::error::Error;

// Pure Ollama call function
pub async fn call_ollama(model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let request_body = GenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        format: Some("json".to_string()),
    };

    // Perform the HTTP request
    let response = client.post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?;

    // Collect the full response text
    let response_obj = response.json::<OllamaResponse>().await?;
    Ok(response_obj.response.trim().to_owned())
}
