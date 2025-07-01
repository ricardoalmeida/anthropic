use clap::Command;
use std::env;

fn main() {
    let matches = Command::new("anthropic")
        .version("0.1.0")
        .about("🤖 Anthropic CLI - Access Anthropic API from the command line")
        .subcommand(
            Command::new("models")
                .about("Manage and view Anthropic models")
                .subcommand(
                    Command::new("list")
                        .about("List all available Anthropic models")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("models", models_matches)) => {
            match models_matches.subcommand() {
                Some(("list", _)) => {
                    list_models();
                }
                _ => {
                    eprintln!("Unknown models subcommand. Use 'anthropic models list'");
                }
            }
        }
        _ => {
            eprintln!("No command specified. Use 'anthropic models list' to get started.");
        }
    }
}

fn list_models() {
    // Check if API key is available
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: ANTHROPIC_API_KEY environment variable not set");
            eprintln!("Please set your API key: export ANTHROPIC_API_KEY=your_key_here");
            return;
        }
    };

    println!("🔍 Fetching available Anthropic models...");
    
    // Make the HTTP request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.anthropic.com/v1/models")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>() {
                    Ok(json) => {
                        if let Some(data) = json["data"].as_array() {
                            println!("\n📋 Available Models:");
                            println!("{:-<60}", "");
                            for model in data {
                                let id = model["id"].as_str().unwrap_or("Unknown ID");
                                let display_name = model["display_name"].as_str().unwrap_or("Unknown Name");
                                println!("• {} - {}", id, display_name);
                            }
                        } else {
                            eprintln!("❌ Unexpected response format");
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to parse response: {}", e);
                    }
                }
            } else {
                eprintln!("❌ API request failed with status: {}", resp.status());
                if let Ok(text) = resp.text() {
                    eprintln!("Response: {}", text);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Network error: {}", e);
        }
    }
}
