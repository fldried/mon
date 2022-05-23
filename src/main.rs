use std::env;
use std::panic;

use serde_json::Value;
use colored::*;
use titlecase::titlecase;
use rand::Rng;

const BLACKLIST: [&'static str; 7] = ["gourgeist", "eiscue", "indeedee", "landorus", "thundurus", "tornadus", "zygarde"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    // remove the first argument (the name of the program) IFF there is more than one argument
    let args = &args[1..];

    //println!("ARGS: {}", args.join(" "));

    panic::set_hook(Box::new(|_info| {}));

    // bool shiny true if there is arg '-s'
    let shiny = args.contains(&String::from("-s"));

    //println!("SHINY: {}", shiny);

    // identifier is the OTHER arg except args[0]
    let identifier = if args.len() == 1 && !shiny || args.len() == 2 && shiny {
        args.iter().find(|x| x != &&String::from("-s")).unwrap().to_string()
    } else {
        "".to_string()
    };

    //println!("IDENTIFIER: {}", identifier.green().bold());

    // make identifier mutable
    let mut identifier = identifier;

    //println!("{}", identifier.green().bold());

    // if identifier is empty, use random number as string
    identifier = if identifier.is_empty() {
        rand::thread_rng().gen_range(1..899).to_string()
    } else {
        identifier
    };

    println!("Generated?: {}", identifier.green().bold());

    if BLACKLIST.contains(&identifier.as_str()) {
        match &*identifier {
            "gourgeist" => identifier += "-average",
            "eiscue" => identifier += "-ice",
            "indeedee" => identifier += "-male",
            "landorus" | "thundurus" | "tornadus" => identifier += "-incarnate",
            "zygarde" => identifier += "-50",
            _ => {
                eprintln!("Argument matched blacklist but did not match a value? Please make an issue w/ the Pokémon's name or ID.");
                panic!();
            }
        };
    }

    println!("{}", identifier.green().bold());

    let request_text = get_pokemon_info(&identifier.to_lowercase()).await?;

    // one match should handle both requests as they use the same name
    match parse_pokemon_info(&request_text).await {
        Ok(p) => {
            let pokemon = p;   
            let colorscript = get_pokemon_colorscript(&pokemon.name, shiny).await?;

            print_pokemon(&pokemon, &colorscript, shiny).await;
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
            x.replace("\"", "")
        },

        types: {
            let mut x: Vec<String> = Vec::new();
            
            x.push(titlecase(&v["types"][0]["type"]["name"].to_string()).replace("\"", ""));

            // try to add the second pokemon's type if it has one
            let check_double = &v["types"][1]["type"]["name"];

            if *check_double != Value::Null {
                x.push(titlecase(&v["types"][1]["type"]["name"].to_string()).replace("\"", ""));
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

async fn get_pokemon_colorscript(name: &String, shiny: bool) -> reqwest::Result<Vec<String>> {
    let name_fixed = match name.as_str() {
        "gourgeist" => identifier += "-average",
        "eiscue" => identifier += "-ice",
        "indeedee" => identifier += "-male",
        "landorus-incarnate" | "thundurus-incarnate" | "tornadus-incarnate" => name.replace("-incarnate", ""),
        "zygarde-50" => name.replace("-50", ""),
        _ => name.to_string()
    };

    let url = if shiny {
        format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/shiny/{}", name_fixed)
    }
    else {
        format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/regular/{}", name_fixed)
    };

    let res = reqwest::get(url).await?;

    let text = res.text().await?;
    let text_lines = text.lines();

    let mut vec: Vec<String> = Vec::new();
    for x in text_lines {
        vec.push(x.to_owned());
    }

    Ok(vec)
}

// return rgb values for each color
async fn get_type_color(type_name: &String) -> Vec<u8> {
    match type_name.as_str() {
        "Normal" => vec![168, 167, 122],    // A8A77A
        "Fire" => vec![238, 129, 48],       // EE8130
        "Water" => vec![99, 144, 240],      // 6390F0
        "Electric" => vec![247, 208, 44],   // F7D02C
        "Grass" => vec![122, 199, 76],      // 7AC74C
        "Ice" => vec![150, 217, 214],       // 96D9D6
        "Fighting" => vec![194, 46, 40],    // C22E28
        "Poison" => vec![163, 62, 161],     // A33EA1
        "Ground" => vec![226, 191, 101],    // E2BF65
        "Flying" => vec![169, 143, 243],    // A98FF3
        "Psychic" => vec![249, 85, 135],    // F95587
        "Bug" => vec![166, 185, 26],        // A6B91A
        "Rock" => vec![182, 161, 54],       // B6A136
        "Ghost" => vec![115, 87, 151],      // 735797
        "Dragon" => vec![111, 53, 252],     // 6F35FC
        "Dark" => vec![112, 87, 70],        // 705746
        "Steel" => vec![183, 183, 206],     // B7B7CE
        "Fairy" => vec![214, 133, 173],     // D685AD
        _ => vec![255, 255, 255]
    }
}

async fn print_pokemon(pokemon: &Pokemon, colorscript: &Vec<String>, shiny: bool) {
    // start printing the info 1/3 of the way through the rendering of the colorscript
    let is = colorscript.len() / 3;
    let indices = [is, is + 1, is + 3, is + 4]; // is + 6 eventually for the synopsis

    let info = [
        format!(
            "{} #{}", 
            if shiny {titlecase(&pokemon.name.replace("-", " ")).bold().white()} else {titlecase(&pokemon.name.replace("-", " ")).bold().black()},
            pokemon.id.to_string().italic().white()
        ),

        // format the types
        // color the types according to the type's color
        format!(
            "{}", {
                let mut x = String::new();
                for (i, t) in pokemon.types.iter().enumerate() {
                    let color = get_type_color(t).await;
                    x += &format!("{}", t.bold().truecolor(color[0], color[1], color[2]));
                    if i != pokemon.types.len() - 1 {
                        x += " / ";
                    }
                }
                x
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
    for i in 0..colorscript.len() {
        if indices.contains(&i) {
            println!("{}\t{}", colorscript[i], info[info_counter]);
            info_counter += 1;
        } else {
            println!("{}", colorscript[i]);
        }
    }
}
