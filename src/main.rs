mod error;
mod cli;
mod pokemon;
mod display;
mod utility;

use cli::Args;
use pokemon::{PokemonClient};
use display::PokemonDisplay;

use clap::Parser;

#[tokio::main]
async fn main() {
    let matches = Args::parse();
    let client = PokemonClient::new();

    let result = client.get_pokemon_based_on_args(&matches).await;
    match result {
        Ok(pokemon) => {
            let display = PokemonDisplay::new(&pokemon, matches.shiny);
            display.show().await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}