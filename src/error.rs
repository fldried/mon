use thiserror::Error;

#[derive(Error, Debug)]
pub enum PokemonError {
    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
