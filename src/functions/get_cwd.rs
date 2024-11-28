
use async_trait::async_trait;
use std::env;
use std::error::Error;
use ollama_rs::generation::functions::tools::Tool;
use serde_json::Value;

// Define a Tool for the `get_cwd` function
pub struct GetCwdTool;

#[async_trait]
impl Tool for GetCwdTool {
    fn name(&self) -> String {
        "get_cwd".to_string()
    }

    fn description(&self) -> String {
        "Returns the current working directory.".to_string()
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
        get_cwd()
    }
}

fn get_cwd() -> Result<String, Box<dyn Error>> {
    let cwd = env::current_dir().unwrap();
    Ok(cwd.display().to_string())

}
