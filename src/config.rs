use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Add metadata here
pub(crate) struct Config {
    /// Command server address
    #[arg(long, default_value = Config::DEFAULT_COMMAND_SERVER)]
    pub(crate) command_server: String,

    /// Ollama server address
    #[arg(long, default_value = Config::DEFAULT_OLLAMA_SERVER)]
    pub(crate) ollama_server: String,

    /// Ollama server port
    #[arg(long, default_value_t = Config::DEFAULT_OLLAMA_PORT)]
    pub(crate) ollama_port: u16,

    /// Ollama model name
    #[arg(long, default_value = Config::DEFAULT_OLLAMA_MODEL)]
    pub(crate) ollama_model: String,

    /// Embedding model name
    #[arg(long, default_value = Config::DEFAULT_EMBEDDING_MODEL)]
    pub(crate) embedding_model: String,
}

impl Config {
    // Default values are declared as constants
    pub const DEFAULT_COMMAND_SERVER: &'static str = "127.0.0.1:9001";
    pub const DEFAULT_OLLAMA_SERVER: &'static str = "http://localhost";
    pub const DEFAULT_OLLAMA_PORT: u16 = 11434;
    pub const DEFAULT_OLLAMA_MODEL: &'static str = "llama3.2:latest";
    pub const DEFAULT_EMBEDDING_MODEL: &'static str = "nomic-embed-text:latest";
}

// Add a custom Default implementation for Config
impl Default for Config {
    fn default() -> Self {
        Config {
            command_server: Config::DEFAULT_COMMAND_SERVER.to_string(),
            ollama_server: Config::DEFAULT_OLLAMA_SERVER.to_string(),
            ollama_port: Config::DEFAULT_OLLAMA_PORT,
            ollama_model: Config::DEFAULT_OLLAMA_MODEL.to_string(),
            embedding_model: Config::DEFAULT_EMBEDDING_MODEL.to_string(),
        }
    }
}