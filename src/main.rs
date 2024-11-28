use std::error::Error;
use ollama_rs::{
    generation::functions::{request::FunctionCallRequest, tools::Tool, NousFunctionCall},
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use std::sync::Arc;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use crate::functions::get_cwd::GetCwdTool;

mod functions {
    pub mod get_cwd;
}

fn cosine_similarity(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f64 { // Dot product divided by magnitude
    assert_eq!(vec1.len(), vec2.len(), "Vectors must be of the same length");

    let dot_product = vec1.iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| f64::from(a) * f64::from(b))
        .sum::<f64>();

    let magnitude1 = vec1.iter().map(|&a| f64::from(a) * f64::from(a)).sum::<f64>().sqrt();
    let magnitude2 = vec2.iter().map(|&b| f64::from(b) * f64::from(b)).sum::<f64>().sqrt();

    dot_product / (magnitude1 * magnitude2)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama = Ollama::new(
        "http://localhost", 11434); // Point this to your Ollama server

    let mut stdout = stdout();
    let all_tools: Vec<Arc<dyn Tool>> = vec![Arc::new(GetCwdTool)]; // List of all tools

    // Init parser
    let parser = Arc::new(NousFunctionCall::new());

    // Models
    let model_name = "llama3.2:latest"; // LLM
    let embedding_model_name = "nomic-embed-text:latest"; // Embedding model

    loop {
        let mut query_tools: Vec<Arc<dyn Tool>> = vec![];

        // Write a prompt indicator
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        // Read from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim_end();

        // Check if we're trying to exit
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Compare tools descriptions to input query
        for tool in all_tools.iter() {

            // Get embeddings of input
            let input_embedding_request = GenerateEmbeddingsRequest::new(embedding_model_name.into(), vec![input].into());
            let input_embedding_response = ollama.generate_embeddings(input_embedding_request).await.unwrap();

            // Get embeddings of tool descriptions
            let tool_embedding_request  = GenerateEmbeddingsRequest::new(embedding_model_name.into(), EmbeddingsInput::from(tool.description()));
            let tool_embedding_response = ollama.generate_embeddings(tool_embedding_request).await.unwrap();

            // Calculate similarity
            let similarity = cosine_similarity(&input_embedding_response.embeddings.concat(), &tool_embedding_response.embeddings.concat());

            println!("{:.2}%", similarity * 100.0);

            // If tool description is semantically similar to input
            if similarity > 0.8 {
                query_tools.push(tool.clone());
            }

        }

        // Convert input to ChatMessage
        let input_chat_message = ChatMessage::user(input.parse().unwrap());

        let response;

        // If we have a tool that corresponds to input, run that
        if !query_tools.is_empty() {

            let request = FunctionCallRequest::new(model_name.into(), query_tools.clone(), vec![input_chat_message]);
            response = ollama.send_function_call(request, parser.clone()).await?;

        }else{ // Otherwise, send input to LLM

            let request = ChatMessageRequest::new(model_name.into(), vec![input_chat_message]);
            response = ollama.send_chat_messages(request).await?;
        }

        // Handle the response directly
        let content = response.message.unwrap().content;
        stdout.write_all(content.as_bytes()).await?;

    }

    Ok(())
}
