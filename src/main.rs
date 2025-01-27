mod api_db;
mod generate_confirmation;
mod generate_json;
mod match_endpoint;
mod match_fields;
mod models;
mod ollama_client;

use crate::api_db::load_config_to_sled;
use crate::generate_confirmation::generate_confirmation;
use crate::generate_json::generate_json;
use crate::match_endpoint::find_matching_endpoint;
use crate::ollama_client::OllamaClient;
use match_fields::match_fields;
use std::error::Error;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    // Paths to configuration and database
    let config_path = "config.yaml"; // Path to your YAML file
    let db_path = "./endpoint_config_db"; // Path where Sled database will be stored

    // Load configuration into Sled DB
    load_config_to_sled(config_path, db_path, false)?;
    let db = sled::open(db_path)?;

    // Create Ollama client
    let ollama_client = OllamaClient::new();

    // Example prompts
    let prompts = vec![
        "send an email to John@gmail.com which title is new report and body is hello john here is the report",
        "create a ticket with high priority titled server down and description is production server not responding",
        "schedule a meeting tomorrow at 2pm for 1 hour with the dev team to discuss project status",
        "analyze logs for auth-service from january 1st to today with error level",
        "deploy application user-service version 2.1.0 to production with rollback to 2.0.9",
        "generate monthly sales report in PDF format",
        "backup database users with full backup and high compression",
        "process payment of 500 USD from customer 12345 using credit card",
    ];

    for prompt in prompts {
        println!("\nPrompt: {}", prompt);

        // Generate JSON
        match generate_json(prompt).await {
            Ok(json_response) => {
                // Pretty print the JSON
                println!(
                    "Generated JSON:\n{}",
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

                            // Add field matching
                            info!("Attempting to match fields for endpoint: {}", endpoint.id);
                            match match_fields(&json_response, &endpoint, &ollama_client).await {
                                Ok(mapped_fields) => {
                                    println!("\nMapped Fields:");
                                    println!("{}", serde_json::to_string_pretty(&mapped_fields)?);

                                    match generate_confirmation(
                                        &mapped_fields,
                                        &endpoint,
                                    )
                                    .await
                                    {
                                        Ok(confirmation) => {
                                            println!("\nConfirmation:\n{}", confirmation)
                                        }
                                        Err(e) => error!("Error generating confirmation: {}", e),
                                    }
                                }
                                Err(e) => error!("Error matching fields: {}", e),
                            }
                        }
                    }
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
