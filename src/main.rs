use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Write};

// Command-line arguments structure
#[derive(Parser, Debug)]
#[clap(author, version, about = "Rust client for LLM Studio API")]
struct Args {
    /// API Base URL
    #[clap(short, long, default_value = "http://localhost:1234/v1")]
    url: String,

    /// API Key
    #[clap(short, long, default_value = "lm-studio")]
    api_key: String,

    /// Model name
    #[clap(short = 'm', long, default_value = "TheBloke/dolphin-2.2.1-mistral-7B-GGUF")]
    model: String,

    /// Temperature
    #[clap(short, long, default_value = "1.1")]
    temperature: f32,

    /// Max tokens
    #[clap(short = 'l', long, default_value = "140")]
    max_tokens: u32,
}

// Message structure for OpenAI-compatible API
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

// Response structures for OpenAI-compatible API
#[derive(Deserialize, Debug)]
struct ChatCompletionChoice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();

    // Create HTTP client
    let client = Client::new();

    println!("\nWelcome to the Rust LLM Chat Interface!");
    println!("Connected to: {} at {}", args.model, args.url);
    println!("Type your messages and press Enter. Type 'exit' or 'quit' to end the chat.");
    println!("------------------------------------------------------------");

    // Initialize chat history with system message
    let mut history = vec![Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    }];

    loop {
        // Get user input
        print!("\nYou: ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        // Check for exit command
        if user_input.to_lowercase() == "exit" || user_input.to_lowercase() == "quit" {
            println!("\nGoodbye!");
            break;
        }

        // Add user message to history
        history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        // Make API request
        let assistant_response = match chat_completion(&client, &args, &history).await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error making API request: {}", e);
                "Sorry, I encountered an error communicating with the LLM server.".to_string()
            }
        };

        // Add assistant response to history
        println!("\nAssistant: {}", assistant_response);
        history.push(Message {
            role: "assistant".to_string(),
            content: assistant_response,
        });
    }

    Ok(())
}

async fn chat_completion(
    client: &Client,
    args: &Args,
    messages: &[Message],
) -> Result<String, Box<dyn std::error::Error>> {
    let endpoint = format!("{}/chat/completions", args.url);

    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", args.api_key))
        .json(&json!({
            "model": args.model,
            "messages": messages,
            "temperature": args.temperature,
            "max_tokens": args.max_tokens
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status code: {}",
            response.status()
        )
        .into());
    }

    let completion: ChatCompletionResponse = response.json().await?;
    Ok(completion.choices[0].message.content.clone())
}