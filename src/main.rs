use std::env;
use std::panic;

use serde_json::Value;
use colored::*;
use titlecase::titlecase;
use rand::Rng;

const BLACKLIST: [&'static str; 3] = ["gourgeist", "eiscue", "indeedee"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args: Vec<String> = env::args().collect();
    panic::set_hook(Box::new(|_info| {}));

    if args.len() == 1 {
        args.push(rand::thread_rng().gen_range(0..899).to_string());
    }

    // this is the stupidest if statement ever, but it wants &&str so you have to pick your battles
    if BLACKLIST.contains(&&*args[1]) {
        match &*args[1] {
            "gourgeist" => args[1] += "-average",
            "eiscue" => args[1] += "-ice",
            "indeedee" => args[1] += "-male",
            _ => {
                eprintln!("Argument matched blacklist but did not match a value? Please make an issue w/ the Pokémon's name or ID.");
                panic!();
            }
        }
    }

    let request_text = get_pokemon_info(&args[1]).await?.to_lowercase();

    // one match should handle both requests as they use the same name
    match parse_pokemon_info(&request_text).await {
        Ok(p) => {
            let pokemon = p;   
            let colorscript = get_pokemon_colorscript(&pokemon.name).await?;

            print_pokemon(&pokemon, &colorscript).await;
        },
        Err(_) => {
            eprintln!("Error parsing Pokémon data, is your name/ID correct?");
            panic!();
        }
    }
    
    Ok(())
}

struct Pokemon {
    id: u16,
    name: String,
    types: Vec<String>,
    weight: f64,
    height: f64,
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

        name: {
            let x = v["name"].to_string();
            //let x = v["name"].to_string().split("-").collect::<Vec<&str>>()[0].to_string();
            x.replace("\"", "")
        },

        types: {
            let mut x: Vec<String> = Vec::new();
            
            x.push(titlecase(&v["types"][0]["type"]["name"].to_string()).replace("\"", ""));

            // try to add the second pokemon's type if it has one
            let check_double = &v["types"][1]["type"]["name"];

            if *check_double != Value::Null {
                let pre: String = "+ ".to_owned() + &check_double.to_string();
                x.push(titlecase(&pre).replace("\"", ""));
            }

            x
        },

        weight: {
            let x = v["weight"].to_string();
            x.parse::<f64>().unwrap()
        },

        height: {
            let x = v["height"].to_string();
            x.parse::<f64>().unwrap()
        }
    };

    Ok(pokemon)
}

async fn get_pokemon_colorscript(name: &String) -> reqwest::Result<Vec<String>> {
    let name_fixed = match name.as_str() {
        "gourgeist-average" => name.replace("-average", ""),
        "eiscue-ice" => name.replace("-ice", ""),
        "indeedee-male" => name.replace("-male", ""),
        _ => name.to_string()
    };

    let res = reqwest::get(format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/regular/{}", name_fixed)).await?;
    let text = res.text().await?;
    let text_lines = text.lines();

    let mut vec: Vec<String> = Vec::new();
    for x in text_lines {
        vec.push(x.to_owned());
    }

    Ok(vec)
}

async fn print_pokemon(pokemon: &Pokemon, colorscript: &Vec<String>) {
    // start printing the info 1/3 of the way through the rendering of the colorscript
    let is = colorscript.len() / 3;
    let indices = [is, is + 1, is + 3, is + 4]; // is + 6 eventually for the synopsis

    let info = [
        format!(
            "{} (#{})", 
            titlecase(&pokemon.name.replace("-", " ")).bold().red(),
            pokemon.id.to_string().italic().white()
        ), 

        format!("{}", {
                let mut y = String::new();
                for x in &pokemon.types {
                    y += &format!("{} ", x);
                }

                y.green()
            }
        ),

        format!("{}", {
                let mut s = String::from("Height: ");

                s += &format!("{}m", &pokemon.height / 10.0);
                s.white()
            }
        ),

        format!("{}", {
                let mut s = String::from("Weight: ");

                s += &format!("{}kg", &pokemon.weight / 10.0);
                s.white()
            }
        )

        // TODO eventually add synopsis
    ];

    let mut info_counter = 0;
    for i in 0..colorscript.len() - 1 {
        if indices.contains(&i) {
            println!("{}\t{}", colorscript[i], info[info_counter]);
            info_counter += 1;
        } else {
            println!("{}", colorscript[i]);
        }
    }
}
