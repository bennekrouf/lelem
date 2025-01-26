use crate::models::Endpoint;
use crate::ollama_client::OllamaClient;
use serde_json::Value;
use sled;
use std::error::Error;
use tracing::{debug, error, info, warn};

/// Finds matching endpoint by comparing input JSON with database entries using Ollama
pub async fn find_matching_endpoint(
    input_json: &Value,
    db: &sled::Db,
    ollama_client: &OllamaClient,
) -> Result<Vec<Endpoint>, Box<dyn Error>> {
    let input_description = input_json["endpoints"][0]["description"]
        .as_str()
        .unwrap_or("");
    let input_fields = input_json["endpoints"][0]["fields"]
        .as_object()
        .map(|fields| {
            fields
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();

    // Method 1: Tournament-style comparison
    let mut endpoints: Vec<Endpoint> = db
        .iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|(_, value)| serde_json::from_slice(&value).ok())
        .collect();

    if endpoints.is_empty() {
        return Ok(vec![]);
    }

    let mut closest_endpoint = endpoints.remove(0);
    while !endpoints.is_empty() {
        let prompt = format!(
            "Compare endpoints vs input:\n\
            Input: {}\nFields: {}\n\
            Reference: ID: {}, Text: {}, Description: {}\n\
            Candidate: ID: {}, Text: {}, Description: {}\n\
            JSON: {{\"closer_endpoint\": \"reference\" or \"candidate\"}}",
            input_description,
            input_fields,
            closest_endpoint.id,
            closest_endpoint.text,
            closest_endpoint.description,
            endpoints[0].id,
            endpoints[0].text,
            endpoints[0].description
        );

        let response_json = ollama_client.generate_json("llama2", &prompt).await?;
        closest_endpoint = if response_json
            .get("closer_endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or("reference")
            == "candidate"
        {
            endpoints.remove(0)
        } else {
            endpoints.remove(0);
            closest_endpoint
        };
    }

    info!(
        "Tournament winner: {} (ID: {})",
        closest_endpoint.description, closest_endpoint.id,
    );

    Ok(vec![closest_endpoint])
}
