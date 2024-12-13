use async_trait::async_trait;
use std::error::Error;
use std::fs;
use std::path::Path;
use ollama_rs::generation::functions::tools::Tool;
use serde_json::Value;

// Define a Tool for the `cat` (read and print file contents) function
pub struct CatTool;

#[async_trait]
impl Tool for CatTool {
    fn name(&self) -> String {
        "cat".to_string()
    }

    fn description(&self) -> String {
        "Reads the contents of a file.".to_string()
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The path to the file to read."
                }
            },
            "required": ["file_path"]
        })
    }

    async fn call(&self, input: &str) -> Result<String, Box<dyn Error>> {
        self.run(serde_json::from_str(input)?).await
    }

    async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
        // Get the "file_path" parameter from input
        let file_path = input.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or("Invalid input: 'file_path' is required and must be a string")?;

        cat(file_path)
    }
}

fn cat(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Check if the file exists
    let file_path = Path::new(file_path);
    if !file_path.exists() {
        return Err(format!("File not found: {}", file_path.display()).into());
    }
    if !file_path.is_file() {
        return Err(format!("Path is not a file: {}", file_path.display()).into());
    }

    // Read the file contents
    let contents = fs::read_to_string(file_path)?;

    // Print contents to stdout (for user interaction, the caller decides how to handle it)
    println!("{}", contents);

    // Return contents as the result
    Ok(contents)
}