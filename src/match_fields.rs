use crate::{models::Endpoint, ollama_client::OllamaClient};
use serde_json::Value;
use std::error::Error;

pub async fn match_fields(
    input_json: &Value,
    closest_endpoint: &Endpoint,
    ollama_client: &OllamaClient,
) -> Result<Value, Box<dyn Error>> {
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

    let parameters = closest_endpoint
        .parameters
        .iter()
        .map(|p| format!("{}: {}", p.name, p.description))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Map these input fields to endpoint parameters:\n\
        Input: {}\n\
        Parameters:\n{}\n\
        Return only a JSON with parameter names as keys and matched values",
        input_fields, parameters
    );

    let response = ollama_client.generate_json("llama2", &prompt).await?;
    Ok(response)

    //let input_fields = input_json["endpoints"][0]["fields"]
    //    .as_object()
    //    .map(|fields| {
    //        fields
    //            .iter()
    //            .map(|(k, v)| format!("{}: {}", k, v))
    //            .collect::<Vec<_>>()
    //            .join(", ")
    //    })
    //    .unwrap_or_default();
    //
    //let prompt = format!(
    //    "Map input fields to endpoint parameters:\n\
    //    Input fields: {}\n\
    //    Endpoint parameters: {:?}\n\
    //    Format response as endpoint JSON with mapped values",
    //    input_fields, closest_endpoint.parameters
    //);
    //
    //let response_json = ollama_client.generate_json("llama2", &prompt).await?;
    //let mapped_endpoint: Endpoint = serde_json::from_value(response_json)?;
    //
    //Ok(mapped_endpoint)
}
