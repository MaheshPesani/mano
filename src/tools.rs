// src/tools.rs
use rig::tool::Tool;
use rig::completion::ToolDefinition; // Fix: Import from completion module
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
pub struct ReadFileArgs {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadFileError {
    pub error: String,
}

impl fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for ReadFileError {}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    const NAME: &'static str = "read_file";
    
    type Error = ReadFileError;
    type Args = ReadFileArgs;
    type Output = String;

    /// Describes the tool's schema (name, description, args) that gets sent to the LLM.
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "read_file".to_string(),
            description: "Reads the contents of a local file.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The absolute or relative path to the file"
                    }
                },
                "required": ["path"]
            }),
        }
    }

    /// Reads the file at `args.path` into a string, wrapping any I/O failure in `ReadFileError`.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        fs::read_to_string(&args.path).map_err(|e| ReadFileError {
            error: format!("Failed to read {}: {}", args.path, e),
        })
    }
}

// --- Run Command Tool ---

#[derive(Deserialize)]
pub struct RunCommandArgs {
    pub command: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunCommandError {
    pub error: String,
}

impl fmt::Display for RunCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for RunCommandError {}

pub struct RunCommandTool;

impl Tool for RunCommandTool {
    const NAME: &'static str = "run_command";
    
    type Error = RunCommandError;
    type Args = RunCommandArgs;
    type Output = String;

    /// Describes the tool's schema (name, description, args) that gets sent to the LLM.
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "run_command".to_string(),
            description: "Executes a shell command on the user's machine and returns the output. Use this to run scripts, list directories, or interact with the OS.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    }
                },
                "required": ["command"]
            }),
        }
    }

    /// Spawns `args.command` via the platform shell (cmd on Windows, sh elsewhere) and captures stdout/stderr.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &args.command])
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&args.command)
                .output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
                Ok(combined_output)
            },
            Err(e) => Err(RunCommandError {
                error: format!("Failed to spawn command process: {}", e),
            }),
        }
    }
}

// --- Write File Tool ---

#[derive(Deserialize)]
pub struct WriteFileArgs {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WriteFileError {
    pub error: String,
}

impl fmt::Display for WriteFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for WriteFileError {}

pub struct WriteFileTool;

impl Tool for WriteFileTool {
    const NAME: &'static str = "write_file";
    
    type Error = WriteFileError;
    type Args = WriteFileArgs;
    type Output = String;

    /// Describes the tool's schema (name, description, args) that gets sent to the LLM.
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "write_file".to_string(),
            description: "Writes content to a local file. Overwrites the file if it already exists. Use this to create or update code files based on user requests.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The absolute or relative path to the file"
                    },
                    "content": {
                        "type": "string",
                        "description": "The actual text or code content to write into the file"
                    }
                },
                "required": ["path", "content"]
            }),
        }
    }

    /// Writes `args.content` to `args.path`, creating the file if needed and overwriting it if it exists.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // fs::write automatically creates the file if it doesn't exist, and overwrites if it does
        fs::write(&args.path, &args.content).map_err(|e| WriteFileError {
            error: format!("Failed to write to {}: {}", args.path, e),
        })?;
        
        Ok(format!("Successfully wrote {} bytes to {}", args.content.len(), args.path))
    }
}