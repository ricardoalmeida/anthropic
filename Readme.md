# Anthropic CLI

[![Anthropic CLI CI](https://github.com/USERNAME/warp_anthropic/actions/workflows/ci.yml/badge.svg)](https://github.com/USERNAME/warp_anthropic/actions/workflows/ci.yml)

A command-line tool for interacting with the Anthropic API. Built in Rust, this CLI allows you to chat with Claude models and explore available model options.

## Features

- 🔍 List available Anthropic models
- 💬 Chat with Claude models
- 🔄 Multi-turn conversations with context persistence
- 🏴‍☠️ Pirate mode! (Arrrr!)

## Installation

### Prerequisites

- Rust and Cargo installed
- Anthropic API key

### Building from source

```bash
# Clone the repository
git clone https://github.com/USERNAME/warp_anthropic.git
cd warp_anthropic

# Build the project
cargo build --release

# The binary will be in target/release/anthropic
```

## Usage

First, set your Anthropic API key:

```bash
export ANTHROPIC_API_KEY=your_api_key_here
```

### List Available Models

```bash
anthropic models list
```

### Chat with Claude

Basic chat:
```bash
anthropic chat "Hello, how are you today?"
```

With a specific model:
```bash
anthropic chat "Tell me about quantum computing" --model claude-opus-4-20250514
```

### Multi-turn Conversations

Save a conversation:
```bash
anthropic chat "What is machine learning?" --output conversation.json
```

Continue a conversation:
```bash
anthropic chat "Can you provide some examples?" --context conversation.json --output conversation.json
```

### Pirate Mode

Chat with a pirate-speaking Claude:
```bash
anthropic chat "What is recursion?" --pirate
```

Save and continue pirate conversations:
```bash
anthropic chat "Explain binary trees" --pirate --output pirate_chat.json
anthropic chat "How about sorting algorithms?" --context pirate_chat.json --output pirate_chat.json
```

## CI/CD

This project uses GitHub Actions for continuous integration:
- Building on multiple platforms (Linux, macOS, Windows)
- Running tests
- Code formatting checks
- Security audits

## License

[MIT License](LICENSE)
