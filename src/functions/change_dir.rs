use async_trait::async_trait;
use std::env;
use std::error::Error;
use std::path::Path;
use ollama_rs::generation::functions::tools::Tool;
use serde_json::Value;

// Define a Tool for the `cd` (change directory) function
pub struct CdTool;

#[async_trait]
impl Tool for CdTool {
    fn name(&self) -> String {
        "cd".to_string()
    }

    fn description(&self) -> String {
        "Change the current working directory. Use this function to change directories.".to_string()
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path of the directory to change to."
                }
            },
            "required": ["path"]
        })
    }

    async fn call(&self, input: &str) -> Result<String, Box<dyn Error>> {
        self.run(serde_json::from_str(input)?).await
    }

    async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
        // Get the "path" parameter from input
        let path = input.get("path")
            .and_then(|v| v.as_str())
            .ok_or("Invalid input: 'path' is required and must be a string")?;

        cd(path)
    }
}

fn cd(path: &str) -> Result<String, Box<dyn Error>> {

    // Use std::env::set_current_dir to change the working directory
    env::set_current_dir(Path::new(path))?;

    Ok(format!("Changed current directory to {}", path))
}