// src/main.rs
mod chat;
mod tools;

use clap::Parser;
use rig::providers::{anthropic, openai};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::chat::run_chat_loop;
use crate::tools::{ReadFileTool, RunCommandTool, WriteFileTool};

#[derive(Parser)]
#[command(name = "mano", about = "Mano - Agentic AI Developer CLI assistant")]
struct Cli {
    #[arg(short, long, default_value = "openai")]
    provider: String,
    
    #[arg(short, long, default_value = "gpt-4o")]
    model: String,
}

// Struct to represent the JSON settings file
#[derive(Deserialize, Debug)]
struct Settings {
    openai_key: String,
    openai_url: String,
    anthropic_key: String,
    anthropic_url: String,
}

// Helper function to load settings from ~/.mano/settings.json
fn load_settings() -> Result<Settings, Box<dyn std::error::Error>> {
    let mut path: PathBuf = dirs::home_dir().ok_or("Could not locate home directory")?;
    path.push(".mano");
    path.push("settings.json");

    if !path.exists() {
        return Err(format!(
            "Settings file not found at {:?}. Please create it with your API keys and URLs.",
            path
        ).into());
    }

    let content = fs::read_to_string(&path)?;
    let settings: Settings = serde_json::from_str(&content)?;
    
    Ok(settings)
}

/// Parses CLI args, loads settings, builds the selected provider's agent, and starts the chat loop.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Load the connection strings from ~/.mano/settings.json
    let settings = match load_settings() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Configuration Error: {}", e);
            std::process::exit(1);
        }
    };

    let preamble = "You are a senior AI Agentic Engineer. You can read files and run shell commands to understand context and accomplish tasks.";

    if cli.provider == "openai" {
        // Pass the dynamically loaded key and URL
        let client = openai::Client::from_url(&settings.openai_key, &settings.openai_url);
        let agent = client.agent(&cli.model)
            .preamble(preamble)
            .tool(ReadFileTool)
            .tool(RunCommandTool)
            .tool(WriteFileTool)
            .build();
            
        // FIX: Pass the provider and model to the chat loop
        run_chat_loop(agent, &cli.provider, &cli.model).await;
        
    } else {
        let client = anthropic::Client::new(
            &settings.anthropic_key, 
            &settings.anthropic_url, 
            None,            
            "2023-06-01"     
        );  
        
        let agent = client.agent(&cli.model)
            .preamble(preamble)
            .max_tokens(4096)
            .tool(ReadFileTool)
            .tool(RunCommandTool)
            .tool(WriteFileTool)
            .build();
            
        // FIX: Pass the provider and model to the chat loop
        run_chat_loop(agent, &cli.provider, &cli.model).await;
    }

    Ok(())
}