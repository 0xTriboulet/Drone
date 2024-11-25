use std::error::Error;
use ollama_rs::{
    generation::functions::{request::FunctionCallRequest, tools::Tool, NousFunctionCall},
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use async_trait::async_trait;
use std::sync::Arc;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::MessageRole::User;
use crate::functions::get_cwd::GetCwdTool;

mod functions {
    pub mod get_cwd;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let mut stdout = stdout();
    let tools: Vec<Arc<dyn Tool>> = vec![Arc::new(GetCwdTool)];
    let parser = Arc::new(NousFunctionCall::new());

    let model_name = "llama3.2:latest";

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim_end();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let input_chat_message = ChatMessage::new(User, input.parse().unwrap());

        let request = FunctionCallRequest::new(model_name.into(), tools.clone(), vec![input_chat_message]);

        // Handle the response directly
        let response = ollama.send_function_call(request, parser.clone()).await?;


        if let content = response.message.unwrap().content {
            stdout.write_all(content.as_bytes()).await?;
        }

    }

    Ok(())
}
