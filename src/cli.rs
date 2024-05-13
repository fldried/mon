use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "A CLI tool to display Pokémon data")]
pub struct Args {
    #[arg(help = "The Pokémon to display (Pokédex number or name)")]
    pub identifier: Option<String>,

    #[arg(short, long, help = "Displays the shiny variant of the Pokémon")]
    pub shiny: bool,

    #[arg(short, long, help = "The generation of the Pokémon to display (1 - 8)", value_name = "Generation")]
    pub gen: Option<u8>,
}