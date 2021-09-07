use std::env;
use std::num::ParseFloatError;

use serde_json::Value;
use colored::*;
use titlecase::titlecase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let request_text = get_pokemon_info(&args[1]).await?;
    let pokemon = parse_pokemon_info(&request_text).await?;
    let colorscript = get_pokemon_colorscript(&pokemon.name).await?;

    print_pokemon(&pokemon, &colorscript).await?;

    Ok(())
}

struct Pokemon {
    id: u16,
    name: String,
    types: Vec<String>,
    weight: String,
    height: String,
}

async fn get_pokemon_info(identifier: &String) -> reqwest::Result<String> {
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", identifier)).await?;
    let text = res.text().await?;

    Ok(text)
}

async fn parse_pokemon_info(info: &String) -> serde_json::Result<Pokemon> {
    let v: Value = serde_json::from_str(&info)?;

    let pokemon = Pokemon {
        id: {
            let x = v["id"].to_string();
            x.parse::<u16>().unwrap()
        },
        name:  v["name"].to_string().to_lowercase().replace("\"", ""),
        types: {
            // take the JSON object that contains the types and extract the first one
            let mut x: Vec<String> = Vec::new();
            
            x.push(titlecase(&v["types"][0]["type"]["name"].to_string()).replace("\"", ""));

            // if the pokemon has a second type then append + to the previous one, which eventually prints something like
            // Grass + Poison, of course replace all hanging ""s and unneeded characters
            if v["types"][1]["type"]["name"] != Value::Null {
                let pre: String = "+ ".to_owned() + &v["types"][1]["type"]["name"].to_string();
                x.push(titlecase(&pre).replace("\"", ""));
            }

            x
        },
        // hectogram to kilogram
        weight: v["weight"].to_string(),
        // decimeter to meter
        height: v["height"].to_string()
    };

    Ok(pokemon)
}

async fn get_pokemon_colorscript(name: &String) -> reqwest::Result<Vec<String>> {
    let res = reqwest::get(format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/{}.txt", name)).await?;
    let text = res.text().await?;
    let text_lines = text.lines();

    let mut vec: Vec<String> = Vec::new();
    for x in text_lines {
        vec.push(x.to_owned());
    }


    Ok(vec)
}

// See previous TODO comment
async fn print_pokemon(pokemon: &Pokemon, colorscript: &Vec<String>) -> Result<(), ParseFloatError> {
    let info_start = colorscript.len() / 3;
    let indices = [info_start, info_start + 1, info_start + 3, info_start + 4]; // info_start + 6 eventually

    // even I don't know how this works...
    let hit_index = 
    [
        // first index prints pokemon name (red, bold) with id number (white, italics)
        format!(
            "{} (#{})", 

            titlecase(&pokemon.name).bold().red(), 
            (pokemon.id).to_string().italic().white()
        ), 

        {
            let mut y = String::new();
            for x in &pokemon.types {
                y += &format!("{} ", x);
            }

            format!("{}", y.green())
        },

        format!("{}", {
                let mut s = String::from("Height: ");

                let x = &pokemon.height.parse::<f64>()?;
                let x = &(x / 10.0);

                s += &format!("{}m", x);

                s.white()
            }
        ),

        format!("{}", {
                let mut s = String::from("Weight: ");

                let x = &pokemon.weight.parse::<f64>()?;
                let x = &(x / 10.0);

                s += &format!("{}kg", x);

                s.white()
            }
        )

        // TODO eventually add synopsis
    ];

    let mut hit_counter = 0;
    for i in 0..colorscript.len() - 1 {
        if indices.contains(&i) {
            println!("{}\t{}", colorscript[i], hit_index[hit_counter]);
            hit_counter = hit_counter + 1;
        } else {
            println!("{}", colorscript[i]);
        }
    }

    Ok(())
}