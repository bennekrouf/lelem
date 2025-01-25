use serde_json::Value;
use sled;
use std::error::Error;
use tracing::{info, debug, warn, error};
use crate::ollama_client::OllamaClient;
use crate::models::Endpoint;

/// Finds matching endpoint by comparing input JSON with database entries using Ollama
pub async fn find_matching_endpoint(
    input_json: &Value, 
    db: &sled::Db, 
    ollama_client: &OllamaClient
) -> Result<Vec<Endpoint>, Box<dyn Error>> {
    // Extract the input description and fields
    let input_description = input_json["endpoints"][0]["description"]
        .as_str()
        .unwrap_or("");
    
    // Extract all fields as a formatted string
    let input_fields = input_json["endpoints"][0]["fields"]
        .as_object()
        .map(|fields| {
            fields.iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<String>>()
                .join(", ")
        })
        .unwrap_or_default();

    info!("Starting endpoint matching process");
    debug!("Input Description: {}", input_description);
    debug!("Input Fields: {}", input_fields);

    // Open the database
    let db_entries: Vec<_> = db.iter()
        .filter_map(|entry| entry.ok())
        .collect();
    
    info!("Total entries in Sled database: {}", db_entries.len());

    // Deserialize all endpoints
    let mut endpoints: Vec<Endpoint> = db_entries.iter()
        .filter_map(|(_, value)| 
            serde_json::from_slice(value).ok()
        )
        .collect();

    // If no endpoints, return empty
    if endpoints.is_empty() {
        warn!("No endpoints found in database");
        return Ok(vec![]);
    }

    // Start with the first endpoint as the reference
    let mut closest_endpoint = endpoints.remove(0);
    
    // Iteratively compare and find the closest match
    while !endpoints.is_empty() {
        // Prepare prompt for comparison
        let prompt = format!(
            "Compare these two endpoint details in the context of the input description and fields:\n\n\
            Input Context:\n\
            - Description: {}\n\
            - Fields: {}\n\n\
            Reference Endpoint:\n\
            - ID: {}\n\
            - Text: {}\n\
            - Description: {}\n\n\
            Candidate Endpoint:\n\
            - ID: {}\n\
            - Text: {}\n\
            - Description: {}\n\n\
            Task: Determine which endpoint is more semantically similar to the input.\n\
            Consider:\n\
            1. Description match\n\
            2. Action similarity\n\
            3. Potential to handle the input fields\n\n\
            Respond ONLY with a JSON:\n\
            {{\n\
              \"closer_endpoint\": \"reference\" or \"candidate\",\n\
              \"similarity_reasoning\": \"Explain why this endpoint is closer\"\n\
            }}",
            input_description,
            input_fields,
            closest_endpoint.id,
            closest_endpoint.text,
            closest_endpoint.description,
            endpoints[0].id,
            endpoints[0].text,
            endpoints[0].description
        );

        // Log the current comparison
        info!(
            "Comparing:\n- Reference Endpoint: {} (ID: {})\n- Candidate Endpoint: {} (ID: {})", 
            closest_endpoint.description, 
            closest_endpoint.id,
            endpoints[0].description,
            endpoints[0].id
        );

        // Call Ollama to compare
        let response_json = ollama_client.generate_json("llama2", &prompt).await?;

        // Determine which endpoint is closer
        let closer_endpoint = response_json.get("closer_endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or("reference");

        // Update closest endpoint
        closest_endpoint = if closer_endpoint == "candidate" {
            endpoints.remove(0)
        } else {
            endpoints.remove(0);
            closest_endpoint
        };
    }

    // Return the final closest endpoint
    info!(
        "Final closest matching endpoint: {} (ID: {})",
        closest_endpoint.description, 
        closest_endpoint.id
    );

    Ok(vec![closest_endpoint])
}
