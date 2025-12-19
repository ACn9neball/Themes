use capitalize::Capitalize;
use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::BufReader, process::Command};

const APP_NAME: &str = "theme";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
struct Interface {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Deserialize, Serialize)]
struct Theme {
    title: String,
    directory: String,
    current: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_alias = "-r")]
    Random,
    #[command(visible_alias = "-d")]
    Display { name: String },
    #[command(visible_alias = "-a")]
    All,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = dirs::config_dir().ok_or("No system config file")?;
    config_path.push(APP_NAME);
    fs::create_dir_all(&config_path)?;
    config_path.push(CONFIG_FILE_NAME);

    let file = File::open(&config_path)?;
    let reader = BufReader::new(file);
    let mut themes: Vec<Theme> = serde_json::from_reader(reader)?;
    let control = Interface::parse();
    let mut update = false;

    match &control.command {
        Commands::Random => {
            let random = rand::thread_rng().gen_range(0..=themes.len());
            let theme = &themes[random];
            bash(&theme.directory);
            bash("/home/n9neball/Scripts/theme_colors.sh");
            for i in 0..=themes.len() - 1 {
                if i != random {
                    themes[i].current = false;
                } else {
                    themes[i].current = true;
                }
            }
            update = true;
        }

        Commands::Display { name } => {
            let mut found = false;
            for theme in themes.iter_mut() {
                if theme.title == name.to_lowercase() {
                    theme.current = true;
                    bash(&theme.directory);
                    bash("/home/n9neball/Scripts/themes.sh");
                    found = true;
                    update = true;
                } else {
                    theme.current = false;
                }
            }

            if !found {
                println!("Theme does not exist");
            }
        }
        Commands::All => {
            let mut count = 0;
            let mut current: Option<&str> = None;
            for theme in &themes {
                println!("- {}", theme.title.capitalize());
                count = count + 1;
                if theme.current {
                    current = Some(&theme.title);
                }
            }

            println!("\nThemes: {}", count);
            if let Some(value) = current {
                println!("Current Theme: {}", value.capitalize());
            }
        }
    }

    if update {
        let updated = File::create(&config_path)?;
        serde_json::to_writer_pretty(updated, &themes)?;
    }

    Ok(())
}

fn bash(theme: &str) {
    let mut set = Command::new("bash");
    set.arg("-c");
    set.arg(theme);
    set.status().expect("Unable to run script");
}
