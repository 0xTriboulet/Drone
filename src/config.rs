use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Add metadata here
pub(crate) struct Config {
    /// Command server address
    #[arg(long, default_value = "127.0.0.1:9001")]
    pub(crate) command_server: String,

    /// Ollama server address
    #[arg(long, default_value = "http://localhost")]
    pub(crate) ollama_server: String,

    /// Ollama server port
    #[arg(long, default_value_t = 11434)]
    pub(crate) ollama_port: u16,

    /// Ollama model name
    #[arg(long, default_value = "llama3.2:latest")]
    pub(crate) ollama_model: String,

    /// Embedding model name
    #[arg(long, default_value = "nomic-embed-text:latest")]
    pub(crate) embedding_model: String,
}