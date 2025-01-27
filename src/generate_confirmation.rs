use crate::{models::Endpoint, ollama_client::call_ollama};
use serde_json::Value;
use std::error::Error;

pub async fn generate_confirmation(
    json: &Value,
    endpoint: &Endpoint,
) -> Result<String, Box<dyn Error>> {
    let fields = json
        .as_object()
        .map(|fields| {
            fields
                .iter()
                .map(|(k, v)| format!("| {} | {} |", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let prompt = format!(
        "Generate a question asking for user confirmation:\n\
        Action: {}\n\
        Parameters:\n\
        | Field | Value |\n\
        |-------|-------|\n\
        {}\n\
        Return the complete confirmation question directly, no JSON wrapper, but explicitly explaining shortly every field value",
        endpoint.description, fields
    );

    let response = call_ollama(&prompt).await?;
    Ok(response)
}
