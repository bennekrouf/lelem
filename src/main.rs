mod ollama;
mod models;
mod generate_json;

use std::error::Error;
use crate::generate_json::generate_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example prompts
    let prompts = vec![
        "send an email to John@gmail.com which title is new report and body is hello john here is the report",
        "Run an analysis for the GPECS application",
        "Concerning the flight booking of Bennekrouf family from geneva to dubai cancel it for these passengers : Fawzan and Abdallah. But run checkin for these passengers: Saliha and Mohamed"
        // "Create a JSON structure for booking a flight with necessary details.",
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
