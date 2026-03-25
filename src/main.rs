use capitalize::Capitalize;
use chrono::prelude::*;
use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufReader,
    process::Command,
};

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
    #[command(visible_alias = "-n")]
    New(NewArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NewArgs {
    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    path: String,
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
    let exclusion: &[usize] = &[15];

    match &control.command {
        Commands::Random => {
            let current_date: DateTime<Local> = Local::now();
            let date = current_date.format("%m/%d").to_string();
            if date == String::from("12/25") {
                let theme = &themes[15];
                bash_themes(&theme.directory);
                bash("/home/n9neball/Scripts/themes.sh");
                for i in 0..themes.len() {
                    if i != 15 {
                        themes[i].current = false;
                    } else {
                        themes[i].current = true;
                    }
                }
            } else if date == String::from("1/1") {
                let theme = &themes[16];
                bash_themes(&theme.directory);
                bash("/home/n9neball/Scripts/themes.sh");
                for i in 0..themes.len() {
                    if i != 16 {
                        themes[i].current = false;
                    } else {
                        themes[i].current = true;
                    }
                }
            } else {
                let random = random_fn(themes.len(), exclusion);
                let theme = &themes[random];
                bash_themes(&theme.directory);
                bash("/home/n9neball/Scripts/themes.sh");
                for i in 0..themes.len() {
                    if i != random {
                        themes[i].current = false;
                    } else {
                        themes[i].current = true;
                    }
                }
            }
            update = true;
        }

        Commands::Display { name } => {
            let mut found = false;
            for theme in themes.iter_mut() {
                if theme.title == name.to_lowercase() {
                    theme.current = true;
                    bash_themes(&theme.directory);
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
        Commands::New(arg) => {
            let title = arg.title.clone();
            let directory = arg.path.clone();
            let theme: Theme = Theme {
                title: title,
                directory: directory,
                current: false,
            };
            themes.push(theme);
            update = true;
            println!("Added!");
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

fn bash_themes(theme: &str) {
    let status = Command::new("bash")
        .arg("-c")
        .arg(
            r#"
            WALLPAPER="$1"
            swww img -t wipe --transition-angle 90 --transition-duration 5 "$WALLPAPER"
            wallust run "$WALLPAPER"
            wal -i "$WALLPAPER" -n
        "#,
        )
        .arg("bash")
        .arg(theme)
        .status();

    if let Err(e) = status {
        eprintln!("Failed to execute process: {}", e);
    }
}

fn random_fn(tthemes: usize, theme: &[usize]) -> usize {
    loop {
        let random = rand::thread_rng().gen_range(0..tthemes);
        if !theme.contains(&random) {
            return random;
        }
    }
}
