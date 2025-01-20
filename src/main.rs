mod ollama;
mod models;
mod generate_json;
mod api_db;
mod match_endpoint;
mod ollama_client;

use std::error::Error;
use crate::generate_json::generate_json;
use crate::api_db::load_config_to_sled;
use crate::ollama_client::OllamaClient;
use crate::match_endpoint::find_matching_endpoint;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    // Paths to configuration and database
    let config_path = "config.yaml";  // Path to your YAML file
    let db_path = "./endpoint_config_db";  // Path where Sled database will be stored

    // Load configuration into Sled DB
    load_config_to_sled(config_path, db_path, false)?;
    let db = sled::open(db_path)?;

    // Create Ollama client
    let ollama_client = OllamaClient::new();

    // Example prompts
    let prompts = vec![
        "send an email to John@gmail.com which title is new report and body is hello john here is the report",
    ];

    for prompt in prompts {
        println!("\nPrompt: {}", prompt);

        // Generate JSON
        match generate_json("llama2", prompt).await {
            Ok(json_response) => {
                // Pretty print the JSON
                println!("Generated JSON:\n{}", 
                    serde_json::to_string_pretty(&json_response)?
                );

                // Use the generated JSON as input for matching
                match find_matching_endpoint(&json_response, &db, &ollama_client).await {
                    Ok(matches) if !matches.is_empty() => {
                        println!("Matching Endpoints:");
                        for endpoint in matches {
                            println!("- {}: {}", endpoint.id, endpoint.description);
                            // Optionally, print matched parameters
                            println!("  Parameters:");
                            for param in &endpoint.parameters {
                                println!("    - {}: {}", param.name, param.description);
                            }
                        }
                    },
                    Ok(_) => println!("No matching endpoints found."),
                    Err(e) => eprintln!("Error finding matching endpoint: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error generating JSON: {}", e);
            }
        }
    }

    Ok(())
}
