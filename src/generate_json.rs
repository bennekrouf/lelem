use reqwest;
use std::error::Error;

use crate::ollama_client::call_ollama;

pub async fn generate_json(sentence: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let full_prompt = format!(
        "Sentence: {}\n\n\
        Task: Generate a precise, minimal JSON structure based strictly on the sentence.\n\n\
        Rules:\n\
        1. Create an 'endpoints' array with exactly the details from the sentence.\n\
        2. Each endpoint must have:\n\
           - Precise 'description' matching the sentence\n\
           - 'fields' object where EACH key has its EXACT value from the sentence\n\
        3. Do NOT invent additional endpoints or fields\n\
        4. Generate only plain field with its value and not a value a field value as field and a boolean nested in
        5. Use the EXACT values found in the sentence for each field\n\n\
        Example input: 'Send email to Alice at alice@example.com'\n\
        Example output:\n\
        {{\n\
          \"endpoints\": [\n\
            {{\n\
              \"description\": \"Send email\",\n\
              \"fields\": {{\n\
                \"recipient\": \"Alice\",\n\
                \"email\": \"alice@example.com\"\n\
              }}\n\
            }}\n\
          ]\n\
        }}\n\n\
        Now for your sentence: {}",
        sentence, sentence
    );

    let full_response_text = call_ollama(&full_prompt).await?;

    // Attempt to parse the JSON
    let parsed_json: serde_json::Value = serde_json::from_str(&full_response_text)
        .map_err(|_| format!("Failed to parse JSON. Raw response: {}", full_response_text))?;

    Ok(parsed_json)
}

