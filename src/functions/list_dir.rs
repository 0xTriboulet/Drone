use async_trait::async_trait;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use ollama_rs::generation::functions::tools::Tool;
use serde_json::Value;

// Define a Tool for the `ls` (list directory contents) function
pub struct ListDirTool;

#[async_trait]
impl Tool for ListDirTool {
    fn name(&self) -> String {
        "ls".to_string()
    }

    fn description(&self) -> String {
        "Lists all files and directories in the current working directory.".to_string()
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn call(&self, _input: &str) -> Result<String, Box<dyn Error>> {
        self.run(Value::Null).await
    }

    async fn run(&self, _input: Value) -> Result<String, Box<dyn Error>> {
        ls()
    }
}

// Function to perform `ls` operation
fn ls() -> Result<String, Box<dyn Error>> {
    // Get the current working directory
    let cwd: PathBuf = std::env::current_dir()?;

    // Read the directory entries
    let entries = fs::read_dir(cwd)?
        .filter_map(|entry| entry.ok())  // Filter out any entries that encountered an error
        .map(|entry| entry.file_name().to_string_lossy().to_string()) // Convert to strings
        .collect::<Vec<String>>();  // Collect into a vector of strings

    // Join entries into a single string for display
    Ok(entries.join("\n"))
}