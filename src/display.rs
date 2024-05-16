use super::pokemon::Pokemon;
use crate::utility::{get_type_color, get_stat_color};

use futures::future::join_all;
use colored::*;
use textwrap::wrap;
use titlecase::titlecase;

pub struct PokemonDisplay<'a> {
    pokemon: &'a Pokemon,
    shiny: bool,
}

impl<'a> PokemonDisplay<'a> {
    pub fn new(pokemon: &'a Pokemon, shiny: bool) -> Self {
        Self { pokemon, shiny }
    }

    pub async fn show(&self) {
        let colorscript = self.get_colorscript().await.unwrap_or_else(|_| vec![]);
        self.print_pokemon(&colorscript).await;
    }

    async fn get_colorscript(&self) -> Result<Vec<String>, reqwest::Error> {
        let suffix = if self.shiny { "shiny" } else { "regular" };
        let name = self.pokemon.name.to_lowercase();
        // let name = name.split('-').next().unwrap_or(&name);
        let url = format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/{}/{}", suffix, name);
        let mut res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            res = reqwest::get(&format!("https://gitlab.com/phoneybadger/pokemon-colorscripts/-/raw/main/colorscripts/small/{}/{}", suffix, &name.split('-').next().unwrap())).await?;
        }
        Ok(res.text().await?.lines().map(String::from).collect())
    }

    async fn print_pokemon(&self, colorscript: &[String]) {

        let script_length = colorscript.len();

        let start_idx = if script_length >= 13 {
            (script_length - 12) / 2
        } else if script_length >= 10 {
            (script_length - 9) / 2 
        } else {
            0
        };

        let bar_length = if script_length >= 13 {
            16
        } else if script_length >= 10 {
            8
        } else {
            6
        };

        let name = titlecase(&self.pokemon.name.replace("-", " "));
        let name = if self.shiny { format!("✨ {} ✨", name.bold().cyan().blink()) } else { name.bold().white().to_string() };
        let id = ("#".to_owned() + &self.pokemon.id.to_string()).italic().truecolor(128, 128, 128);

        let name_display = format!("{} {}", name, id);

        let types_colored: Vec<String> = join_all(self.pokemon.types.iter().map(|t| {
            async {
                let color = get_type_color(t).await;
                format!("{}", t.bold().truecolor(color[0], color[1], color[2]))
            }
        })).await.into_iter().collect();

        let types_display = types_colored.join(" / ");

        let height_display = format!("{}: {:}m", "Height".truecolor(128,128,128), self.pokemon.height.to_string().white());
        let weight_display = format!("{}: {:}kg", "Weight".truecolor(128,128,128), self.pokemon.weight.to_string().white());

        let stats_display = join_all(self.pokemon.stats.iter().map(|stat| async {
            let filled_length = ((stat.value as f64 / 255.0) * bar_length as f64).round() as usize;
            let empty_length = bar_length - filled_length;
            let color = get_stat_color(stat.value).await;
            let filled_bar = "\u{2593}".repeat(filled_length).truecolor(color[0], color[1], color[2]).bold();
            let empty_bar = "\u{2593}".repeat(empty_length).truecolor(50, 50, 50);
            format!("{:<3}: {:>3} {}{}", stat.name.to_string().truecolor(128, 128, 128), stat.value.to_string().white(), filled_bar, empty_bar)
        })).await.into_iter().collect::<Vec<_>>();

        let description = format!("{}: {}", "Synopsis".truecolor(128,128,128), self.pokemon.flavor_text.replace("♀", " "));
        let description_display = wrap(&description, 55).join("\n");
    
        let mut info = vec![name_display, types_display, "".to_string(), height_display, weight_display, "".to_string()];

        if script_length >= 13 {
            info.extend(stats_display);
        } else if script_length >= 10 {
            let mut new_stats_display = Vec::new();
            for stats_chunk in stats_display.chunks(2) {
                let line = stats_chunk.join("   ");
                new_stats_display.push(line);
            }
            info.extend(new_stats_display);
        } else {
            let mut new_stats_display = Vec::new();
            for stats_chunk in stats_display.chunks(2) {
                let line = stats_chunk.join("   ");
                new_stats_display.push(line);
            }
            info.extend(new_stats_display);
        }

        if script_length < 10 {
            info.remove(2);
            info.remove(4); //index 5, but we removed an element so it's now 4
        }

        println!();
        for (i, line) in colorscript.iter().enumerate() {
            if i >= start_idx && i < start_idx + info.len() {
                println!("{}\t{}", line, info[i - start_idx]);
            } else {
                println!("{}", line);
            }
        }
        println!("{}", description_display);
        println!();
    }
}
