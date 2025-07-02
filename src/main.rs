use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Conversation {
    system_prompt: String,
    model: String,
    messages: Vec<Message>,
}

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
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("File path to save the conversation to")
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("context")
                        .long("context")
                        .short('c')
                        .help("File path to load previous conversation from")
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("pirate")
                        .long("pirate")
                        .short('p')
                        .help("Enables pirate voice mode, arrr!")
                        .action(clap::ArgAction::SetTrue)
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
            let output_path = chat_matches.get_one::<String>("output");
            let context_path = chat_matches.get_one::<String>("context");
            let pirate_mode = chat_matches.get_flag("pirate");
            chat_with_claude(message, model, output_path, context_path, pirate_mode);
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

fn chat_with_claude(message: &str, model: &str, output_path: Option<&String>, context_path: Option<&String>, pirate_mode: bool) {
    // Check if API key is available
    let api_key = match env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: ANTHROPIC_API_KEY environment variable not set");
            eprintln!("Please set your API key: export ANTHROPIC_API_KEY=your_key_here");
            return;
        }
    };

    // Choose system prompt based on mode
    let system_prompt = if pirate_mode {
        println!("🏴‍☠️ Pirate mode enabled! Arrr!");
        "You are Captain Claude, a pirate AI assistant. You must always respond in pirate speak, using pirate slang, nautical terms, and a swashbuckling attitude. Add 'Arr', 'Matey', 'Ahoy', 'Avast', and other pirate phrases throughout your responses. Your knowledge remains accurate, but your manner is that of a salty sea captain who loves adventure, treasure, and the open seas. Use nautical metaphors when explaining complex concepts."
    } else {
        "You are Claude, a helpful AI assistant created by Anthropic. You are knowledgeable, thoughtful, and aim to be helpful while being honest about your limitations. You provide clear, accurate, and well-reasoned responses."
    };
    
    // Load existing conversation or create new one
    let mut conversation = if let Some(context_file) = context_path {
        match load_conversation(context_file) {
            Ok(conv) => {
                println!("📂 Loaded conversation from: {}", context_file);
                println!("📊 Previous messages: {}", conv.messages.len());
                conv
            }
            Err(e) => {
                eprintln!("❌ Failed to load conversation from {}: {}", context_file, e);
                eprintln!("🆕 Starting new conversation instead");
                Conversation {
                    system_prompt: system_prompt.to_string(),
                    model: model.to_string(),
                    messages: Vec::new(),
                }
            }
        }
    } else {
        Conversation {
            system_prompt: system_prompt.to_string(),
            model: model.to_string(),
            messages: Vec::new(),
        }
    };

    // Add the new user message
    let user_message = Message {
        role: "user".to_string(),
        content: message.to_string(),
    };
    conversation.messages.push(user_message);

    println!("💬 Chatting with {} ...", conversation.model);
    println!("👤 You: {}", message);
    
    // Construct the request payload with all messages
    let payload = serde_json::json!({
        "model": conversation.model,
        "max_tokens": 1024,
        "system": conversation.system_prompt,
        "messages": conversation.messages
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
                                    
                                    // Add Claude's response to the conversation
                                    let assistant_message = Message {
                                        role: "assistant".to_string(),
                                        content: text.to_string(),
                                    };
                                    conversation.messages.push(assistant_message);
                                    
                                    // Save conversation if output path is specified
                                    if let Some(output_file) = output_path {
                                        match save_conversation(&conversation, output_file) {
                                            Ok(_) => {
                                                println!("\n💾 Conversation saved to: {}", output_file);
                                            }
                                            Err(e) => {
                                                eprintln!("❌ Failed to save conversation: {}", e);
                                            }
                                        }
                                    }
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

fn load_conversation(file_path: &str) -> Result<Conversation, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let conversation: Conversation = serde_json::from_str(&content)?;
    Ok(conversation)
}

fn save_conversation(conversation: &Conversation, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = serde_json::to_string_pretty(conversation)?;
    
    // Create directory if it doesn't exist
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(file_path, json_content)?;
    Ok(())
}
