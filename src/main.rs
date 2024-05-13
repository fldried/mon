use clap::Parser;
use rand::Rng;
use rand::seq::SliceRandom;
use colored::*;
use titlecase::titlecase;
use textwrap::wrap;

const BASE_URL: &str = "https://pokeapi.co/api/v2";
const COLORSCRIPT_URL: &str = "https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/";

#[derive(Parser, Debug)]
#[command(version, about, long_about = "A CLI tool to display Pokémon data")]
struct Args {
    #[arg(help = "The Pokémon to display (Pokédex number or name)")]
    identifier: Option<String>,

    #[arg(short, long, help = "Displays the shiny variant of the Pokémon")]
    shiny: bool,

    #[arg(short, long, help = "The generation of the Pokémon to display (1 - 8)", value_name = "Generation")]
    gen: Option<u8>,
}

#[derive(Debug)]
struct Stat {
    name: String,
    value: u8,
}

#[derive(Debug)]
struct Pokemon {
    id: u16,
    name: String,
    types: Vec<String>,
    weight: f64,
    height: f64,
    stats: Vec<Stat>,
    flavor_text: String, // Synopsis
}

fn main() {
    let matches = Args::parse();

    let mut rng = rand::thread_rng();
    let input = match matches.gen {
        Some(1) => rng.gen_range(1..=151).to_string(), // Generation 1: 151 Pokemon
        Some(2) => rng.gen_range(152..=251).to_string(), // Generation 2: 100 Pokemon
        Some(3) => rng.gen_range(252..=386).to_string(), // Generation 3: 135 Pokemon
        Some(4) => rng.gen_range(387..=493).to_string(), // Generation 4: 107 Pokemon
        Some(5) => rng.gen_range(494..=649).to_string(), // Generation 5: 156 Pokemon
        Some(6) => rng.gen_range(650..=721).to_string(), // Generation 6: 72 Pokemon
        Some(7) => rng.gen_range(722..=809).to_string(), // Generation 7: 88 Pokemon
        Some(8) => rng.gen_range(810..=898).to_string(), // Generation 8: 89 Pokemon
        _ => match matches.identifier {
            Some(input) => input,
            None => rng.gen_range(1..=898).to_string(), // Default: All generations
        },
    };

    println!();

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match fill_mon_struct(&input.to_lowercase()).await {
            Ok(mon) => {
                let colorscript = get_colorscript(&mon.name, matches.shiny).await.unwrap();
                print_pokemon(&mon, &colorscript, matches.shiny).await;
            },
            Err(_) => {
                eprintln!("Failed to get Pokémon data, is your name or Pokédex Number correct?");
                return;
            }
        }
    });
}

async fn fill_mon_struct(identifier: &str) -> Result<Pokemon, reqwest::Error> {
    let mut mon = Pokemon {
        id: 0,
        name: "".to_string(),
        types: Vec::new(),
        weight: 0.0,
        height: 0.0,
        stats: Vec::new(),
        flavor_text: "".to_string(),
    };
    
    match reqwest::get(format!("{}/pokemon-species/{}", BASE_URL, identifier)).await {
        Ok(res) => {
            let species_json = res.json::<serde_json::Value>().await?;
            mon.name = species_json["varieties"][0]["pokemon"]["name"].as_str().unwrap().to_string();
            mon.flavor_text = {
                let flavor_text_entries = species_json["flavor_text_entries"].as_array().unwrap();
                let mut rng = rand::thread_rng();
                let english_flavor_texts: Vec<_> = flavor_text_entries.iter()
                    .filter(|entry| entry["language"]["name"].as_str().unwrap() == "en")
                    .collect();
                let random_flavor_text = english_flavor_texts.choose(&mut rng)
                    .and_then(|entry| entry["flavor_text"].as_str())
                    .unwrap_or("")
                    .replace("\n", " ");
                random_flavor_text
            };
        },
        Err(e) => {
            eprintln!("Failed to get species data: {}", e);
            return Err(e);
        }
    }

    match reqwest::get(format!("{}/pokemon/{}", BASE_URL, mon.name)).await {
        Ok(res) => {
            let mon_json = res.json::<serde_json::Value>().await?;
            mon.id = mon_json["id"].as_u64().unwrap() as u16;
            mon.types = {
                let types_data = mon_json["types"].as_array().unwrap();
                let mut types = Vec::new();
            
                for type_data in types_data {
                    let type_name = type_data["type"]["name"].as_str().unwrap().to_string();
                    let type_name = titlecase(&type_name);
                    types.push(type_name);
                }
            
                types
            };
            mon.weight = mon_json["weight"].as_f64().unwrap() / 10.0;
            mon.height = mon_json["height"].as_f64().unwrap() / 10.0;
            mon.stats = {
                mon_json["stats"].as_array().unwrap().iter().map(|stat_data| {
                    Stat {
                        name: match stat_data["stat"]["name"].as_str().unwrap() {
                            "hp" => "HP".to_string(),
                            "attack" => "Atk".to_string(),
                            "defense" => "Def".to_string(),
                            "special-attack" => "SpA".to_string(),
                            "special-defense" => "SpD".to_string(),
                            "speed" => "Spe".to_string(),
                            other => other.to_string(),
                        },
                        value: stat_data["base_stat"].as_u64().unwrap() as u8,
                    }
                }).collect::<Vec<Stat>>()
            };
        },
        Err(e) => {
            eprintln!("Failed to get Pokémon data: {}", e);
            return Err(e);
        }
    }

    Ok(mon)
}

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

async fn get_colorscript(name: &str, shiny: bool) -> Result<Vec<String>, reqwest::Error> {
    let url = format!(
        "{}/{}/{}",
        COLORSCRIPT_URL,
        if shiny { "shiny" } else { "regular" },
        name
    );

    let response = reqwest::get(&url).await?;
    if response.status().is_client_error() || response.status().is_server_error() {
        let base_name = name.split('-').next().unwrap();
        return Box::pin(get_colorscript(base_name, shiny)).await;
    }

    let response_text = response.text().await?;

    let lines = response_text.lines();

    let mut vec: Vec<String> = Vec::new();
    for line in lines {
        vec.push(line.to_owned());
    }

    Ok(vec)
}




async fn print_pokemon(mon: &Pokemon, colorscript: &Vec<String>, shiny: bool) {
    let info_start_index = if colorscript.len() >= 14 {
        (colorscript.len() - 13) / 2
    } else {
        0
    };
    let name = titlecase(&mon.name.replace("-", " "));
    let name = if shiny { format!("✨{}✨", name.bold().cyan().blink()) } else { name.bold().white().to_string() };
    let id = ("#".to_owned() + &mon.id.to_string()).italic().black();
    let name_and_id = format!("{} {}", name, id);

    let type_futures: Vec<_> = mon.types.iter().map(|t| get_type_color(t)).collect();
    let type_colors: Vec<_> = futures::future::join_all(type_futures).await;

    let types = mon.types.iter().enumerate().map(|(i, t)| {
        let color = &type_colors[i];
        let type_name = t.bold().truecolor(color[0], color[1], color[2]);
        if i != mon.types.len() - 1 {
            format!("{} / ", type_name)
        } else {
            type_name.to_string()
        }
    }).collect::<String>();

    let height = format!("{} {}m", "Height:".truecolor(128,128,128), mon.height.to_string().white());
    let weight = format!("{} {}kg", "Weight:".truecolor(128,128,128), mon.weight.to_string().white());

    let bar_length = if colorscript.len() < 13 { 8.0 } else { 16.0 };

    let stats = mon.stats.iter().map(|stat| {
        let filled_length = ((stat.value as f64) / 255.0 * bar_length).round() as usize;
        let empty_length = (bar_length - filled_length as f64).round() as usize;
        let mut filled_bar = "█".repeat(filled_length);
        if filled_length >= (bar_length * 4.0 / 5.0) as usize {
            filled_bar = filled_bar.truecolor(0, 255, 0).to_string()
        } else if filled_length >= (bar_length * 3.0 / 5.0) as usize {
            filled_bar = filled_bar.truecolor(0, 192, 0).to_string()
        } else if filled_length >= (bar_length * 2.0 / 5.0) as usize {
            filled_bar = filled_bar.truecolor(255, 192, 0).to_string()
        } else if filled_length >= (bar_length * 1.0 / 5.0) as usize {
            filled_bar = filled_bar.truecolor(242, 140, 40).to_string()
        } else {
            filled_bar = filled_bar.truecolor(255, 0, 0).to_string()
        }
        let empty_bar = "█".repeat(empty_length).truecolor(40, 40, 40);
        format!("{:<3}: {:>3} {}{}", stat.name.truecolor(128,128,128), stat.value, filled_bar, empty_bar)
    }).collect::<Vec<_>>();
    

    let mut info = vec![name_and_id, types, "".to_string(), height, weight, "".to_string()];

    if colorscript.len() < 13 {
        let mut new_stats = Vec::new();
        for stats_chunk in stats.chunks(2) {
            let line = stats_chunk.join("  ");
            new_stats.push(line);
        }
        info.extend(new_stats);
    } else {
        info.extend(stats);
    }

    for (i, line) in colorscript.iter().enumerate() {
        if i >= info_start_index && i < info_start_index + info.len() {
            println!("{}\t{}", line, info[i - info_start_index]);
        } else {
            println!("{}", line);
        }
    }

    let wrapped_synopsis = wrap(&mon.flavor_text, 55);
    for line in wrapped_synopsis {
        println!("{}", line);
    }
}