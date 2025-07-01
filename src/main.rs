use clap::{Arg, Command};
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
        .subcommand(
            Command::new("chat")
                .about("Chat with Claude using the Messages API")
                .arg(
                    Arg::new("message")
                        .help("The message to send to Claude")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("model")
                        .long("model")
                        .short('m')
                        .help("The model to use for the conversation")
                        .default_value("claude-3-5-sonnet-20241022")
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
        Some(("chat", chat_matches)) => {
            let message = chat_matches.get_one::<String>("message").unwrap();
            let model = chat_matches.get_one::<String>("model").unwrap();
            chat_with_claude(message, model);
        }
        _ => {
            eprintln!("No command specified. Try 'anthropic models list' or 'anthropic chat \"Hello!\"'");
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

fn chat_with_claude(message: &str, model: &str) {
    // Check if API key is available
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: ANTHROPIC_API_KEY environment variable not set");
            eprintln!("Please set your API key: export ANTHROPIC_API_KEY=your_key_here");
            return;
        }
    };

    println!("💬 Chatting with {} ...", model);
    println!("👤 You: {}", message);
    
    // Default system prompt for a helpful AI assistant
    let system_prompt = "You are Claude, a helpful AI assistant created by Anthropic. You are knowledgeable, thoughtful, and aim to be helpful while being honest about your limitations. You provide clear, accurate, and well-reasoned responses.";
    
    // Construct the request payload
    let payload = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "system": system_prompt,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ]
    });

    // Make the HTTP request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>() {
                    Ok(json) => {
                        if let Some(content) = json["content"].as_array() {
                            if let Some(first_content) = content.first() {
                                if let Some(text) = first_content["text"].as_str() {
                                    println!("\n🤖 Claude: {}", text);
                                } else {
                                    eprintln!("❌ Could not extract text from response");
                                }
                            } else {
                                eprintln!("❌ No content in response");
                            }
                        } else {
                            eprintln!("❌ Unexpected response format");
                            eprintln!("Response: {}", json);
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
