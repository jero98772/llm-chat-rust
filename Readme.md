# LLM Chat - Rust Client for LLM Studio

A simple command-line tool to chat with local large language models through the LM Studio API server.

## Features

- Clean command-line interface for chatting with local LLMs
- Maintains conversation history for context
- Configurable settings including API endpoint, model, temperature, and token limit
- Compatible with LM Studio's OpenAI-compatible API

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- [LM Studio](https://lmstudio.ai/) with a loaded model and API server running

## Installation

1. Clone this repository or create a new Rust project:

```bash
git clone https://github.com/jero98772/llm-chat-rust
```

2. Navigate to the project directory:

```bash
cd llm-chat
```

3. Build the project:

```bash
cargo build --release
```

The executable will be available at `target/release/llm-chat`.

## Usage

### Basic Usage

Run the application with default settings:

```bash
cargo run
# OR using the built executable
./target/release/llm-chat
```

### Command-Line Options

The application supports the following command-line arguments:

```
USAGE:
    llm-chat [OPTIONS]

OPTIONS:
    -u, --url <URL>                API Base URL [default: http://localhost:1234/v1]
    -k, --api-key <KEY>            API Key [default: lm-studio]
    -m, --model <MODEL>            Model name [default: TheBloke/dolphin-2.2.1-mistral-7B-GGUF]
    -t, --temperature <TEMP>       Temperature [default: 1.1]
    -l, --max-tokens <TOKENS>      Max tokens [default: 140]
    -h, --help                     Print help information
    -V, --version                  Print version information
```

### Example with Custom Options

```bash
cargo run -- -u http://localhost:1234/v1 -k lm-studio -m "mistral-7b-instruct" -t 0.7 -l 200
```

## Setting Up LM Studio

1. Download and install [LM Studio](https://lmstudio.ai/)
2. Open LM Studio and download/select a model
3. Go to the "Local Server" tab
4. Click "Start Server" to start the API server
5. Make sure the API URL in this application matches the one shown in LM Studio

## Chat Interface

Once the application is running:

1. Type your message and press Enter
2. The AI will respond via the API
3. Continue the conversation
4. Type 'exit' or 'quit' to end the chat

## Development

### Dependencies

- `clap`: Command-line argument parsing
- `reqwest`: HTTP client for API requests
- `serde` and `serde_json`: JSON serialization/deserialization
- `tokio`: Async runtime

### Project Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Project configuration and dependencies

## License

[GPLv3](LICENSE)

