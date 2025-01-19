use serde::{Deserialize, Serialize};
use reqwest;
use std::error::Error;

// Structure for the Ollama generate request
#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: Option<String>,
}

// Structure for Ollama's response chunks
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

async fn generate_json(model: &str, prompt: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let request_body = GenerateRequest {
        model: model.to_string(),
        prompt: format!("{}\n\nRespond ONLY with a valid JSON object. Make sure the JSON is parseable.", prompt),
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
    let full_response_text = response_obj.response.trim().to_owned();

    // Attempt to parse the JSON
    let parsed_json: serde_json::Value = serde_json::from_str(&full_response_text)
        .map_err(|_| format!("Failed to parse JSON. Raw response: {}", full_response_text))?;

    Ok(parsed_json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example prompts
    let prompts = vec![
        "Generate a JSON with an array of endpoints for sending an email to John. Each endpoint should have a description and required fields.",
        "Create a JSON structure for booking a flight with necessary details.",
    ];

    for prompt in prompts {
        println!("\nPrompt: {}", prompt);
        match generate_json("llama2", prompt).await {
            Ok(json_response) => {
                // Pretty print the JSON
                println!("Returned JSON:\n{}", 
                    serde_json::to_string_pretty(&json_response)?
                );
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
