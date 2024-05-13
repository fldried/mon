use crate::error::PokemonError;
use crate::cli::Args;

use titlecase::titlecase;
use reqwest::Error;
use serde_json::Value;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Stat {
    pub name: String,
    pub value: u8,
}

pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub types: Vec<String>,
    pub weight: f64,
    pub height: f64,
    pub stats: Vec<Stat>,
    pub flavor_text: String,
}

pub struct PokemonClient {
    base_url: String,
}

impl PokemonClient {
    pub fn new() -> Self {
        Self { base_url: "https://pokeapi.co/api/v2".to_string() }
    }

    pub async fn get_pokemon_based_on_args(&self, args: &Args) -> Result<Pokemon, PokemonError> {
        if let Some(gen) = args.gen {
            let id = self.select_random_pokemon_id(gen).await?;
            self.get_pokemon(&id.to_string()).await.map_err(PokemonError::from)
        } else if let Some(ref identifier) = args.identifier {
            self.get_pokemon(identifier).await.map_err(PokemonError::from)
        } else if let None = args.identifier {
            self.get_pokemon(&self.select_random_pokemon_id(0).await?.to_string()).await.map_err(PokemonError::from)
        } else {
            Err(PokemonError::InvalidInput("No valid identifier or generation specified".to_string()))
        }
    }

    async fn select_random_pokemon_id(&self, generation: u8) -> Result<u16, PokemonError> {
        let (start, end) = match generation {
            1 => (1, 151),
            2 => (152, 251),
            3 => (252, 386),
            4 => (387, 493),
            5 => (494, 649),
            6 => (650, 721),
            7 => (722, 809),
            8 => (810, 898),
            _ => (1, 898),
        };
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(start..=end))
    }

    async fn get_pokemon(&self, identifier: &str) -> Result<Pokemon, Error> {
        let species_url = format!("{}/pokemon-species/{}", self.base_url, identifier);
        let species_res = reqwest::get(&species_url).await?.json::<Value>().await?;

        let name = species_res["varieties"][0]["pokemon"]["name"].as_str().unwrap().to_string();

        let pokemon_url = format!("{}/pokemon/{}", self.base_url, name);
        let pokemon_res = reqwest::get(&pokemon_url).await?.json::<Value>().await?;

        let flavor_texts = species_res["flavor_text_entries"]
            .as_array()
            .map_or_else(Vec::new, |arr| {
                arr.iter()
                .filter(|f| f["language"]["name"].as_str().unwrap_or("") == "en")
                .collect()
            });

        let random_flavor_text = flavor_texts
            .choose(&mut rand::thread_rng())
            .and_then(|f| f["flavor_text"].as_str())
            .unwrap_or("")
            .replace("\n", " ")
            .replace("\u{c}", " ");

        Ok(Pokemon {
            id: pokemon_res["id"].as_u64().unwrap() as u16,
            name: titlecase(&name),
            types: pokemon_res["types"].as_array().unwrap().iter().map(|t| {
                titlecase(t["type"]["name"].as_str().unwrap())
            }).collect(),
            weight: pokemon_res["weight"].as_f64().unwrap() / 10.0,
            height: pokemon_res["height"].as_f64().unwrap() / 10.0,
            stats: pokemon_res["stats"].as_array().unwrap().iter().map(|s| {
                Stat {
                    name: match s["stat"]["name"].as_str().unwrap() {
                        "hp" => "HP".to_string(),
                        "attack" => "Atk".to_string(),
                        "defense" => "Def".to_string(),
                        "special-attack" => "SpA".to_string(),
                        "special-defense" => "SpD".to_string(),
                        "speed" => "Spe".to_string(),
                        other => other.to_string(),
                    },
                    value: s["base_stat"].as_u64().unwrap() as u8,
                }
            }).collect(),
            flavor_text: random_flavor_text,
        })
    }
}
