use serde::{Deserialize, Serialize};
use sled;
use serde_yaml;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Endpoint {
    id: String,
    text: String,
    description: String,
    parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Parameter {
    name: String,
    description: String,
    required: bool,
    alternatives: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    endpoints: Vec<Endpoint>,
}

/// Loads configuration from a YAML file into a Sled database
pub fn load_config_to_sled(config_path: &str, db_path: &str, force: bool) -> Result<(), Box<dyn Error>> {

    // Open or create Sled database
    let db = sled::open(db_path)?;


    // Check if database needs to be cleared or reloaded
    if force {
        // Completely remove and recreate the database
        std::fs::remove_dir_all(db_path)?;
        let db = sled::open(db_path)?;
    }

    // Read YAML file
    let config_content = std::fs::read_to_string(config_path)?;
    let config: ConfigFile = serde_yaml::from_str(&config_content)?;

    // Check if database is empty or force is true
    if db.is_empty() || force {
        // Clear existing data if force is true
        if force {
            db.clear()?;
        }

        // Store each endpoint in the database
        for endpoint in config.endpoints {
            // Convert endpoint to JSON for storage
            let endpoint_json = serde_json::to_vec(&endpoint)?;

            // Store in Sled DB using endpoint ID as key
            db.insert(endpoint.id.as_bytes(), endpoint_json)?;
        }

        // Ensure all data is persisted
        db.flush()?;

        println!("Configuration {} into Sled database successfully.", 
            if force { "forcefully reloaded" } else { "loaded" }
        );
    } else {
        println!("Sled database already contains data. Skipping configuration load.");
    }

    Ok(())
}

/// Retrieves an endpoint from Sled database by its ID
fn get_endpoint(db: &sled::Db, endpoint_id: &str) -> Result<Option<Endpoint>, Box<dyn Error>> {
    match db.get(endpoint_id.as_bytes())? {
        Some(ivec) => {
            let endpoint: Endpoint = serde_json::from_slice(&ivec)?;
            Ok(Some(endpoint))
        },
        None => Ok(None)
    }
}
