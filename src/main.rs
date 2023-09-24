use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::{fs, process};
use std::fs::File;
use std::io::Write;
use std::process::Command;

const CONFIG_FILE_PATH: &str = "/etc/wg-switch/config.json";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    template_path: String,
    wg_config_file_path: String,
    interface: String,
    profiles: HashMap<String, Location>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Location {
    Endpoint: String,
    PublicKey: String,
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "wg-switch")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all profiles
    List,
    /// Switch to a specific profile
    Switch {
        profile: String,
    }
}

fn get_configurations() -> Config {
    let config_data = fs::read_to_string(CONFIG_FILE_PATH);
    let json_data = match config_data {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read configuration file at {CONFIG_FILE_PATH}: {e}");
            process::exit(1);
        },
    };
    return serde_json::from_str(&json_data).unwrap();
}

fn list_profiles(config: Config) {
    println!("List of available profiles");
    for key in config.profiles.keys() {
        println!("{}", key);
    }
}

fn switch_profile(config: Config, profile_name: String) {
    let record_result = config.profiles.get(profile_name.as_str());
    let record = match record_result {
        None => {
            println!("Profile not found: {}", profile_name);
            process::exit(1);
        },
        Some(record) => record,
    };

    let mut template = fs::read_to_string(config.template_path).unwrap();
    template = template.replace("{Endpoint}", &record.Endpoint);
    template = template.replace("{PublicKey}", &record.PublicKey);

    println!("Stopping wg service");
    Command::new("systemctl").arg("stop").arg(format!("wg-quick@{}", config.interface.as_str()))
        .output()
        .expect("Failed to start wg");

    println!("Writing new configurations");
    let mut file = File::create(config.wg_config_file_path).expect("Failed to open configuration file for writing");
    file.write_all(template.as_bytes()).expect("Failed to write configuration file");
    file.sync_all().expect("Failed to flush changes to configuration file");

    println!("Starting wg service");
    Command::new("systemctl").arg("start").arg(format!("wg-quick@{}", config.interface.as_str()))
        .output()
        .expect("Failed to start wg");
}

fn main()  {
    let args = Cli::parse();
    let config = get_configurations();
    match args.command {
        Commands::List => list_profiles(config),
        Commands::Switch { profile } => switch_profile(config, profile),
    }
}
