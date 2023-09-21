use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::{fs, process};
use std::fs::File;
use std::io::Write;
use std::process::Command;


const PROFILES_PATH: &str = "/etc/wireguard/profiles.json";
const TEMPLATE_PATH: &str = "/etc/wireguard/template.conf";
const CONFIG_FILE_PATH: &str = "/etc/wireguard/wg0.conf";

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "wg-switch")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // List all profiles
    List,
    // Switch to a specific profile
    Switch {
        profile: String,
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Location {
    Endpoint: String,
    PublicKey: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    profiles: HashMap<String, Location>
}

fn get_all_profiles() -> HashMap<String, Location> {
    let json_data_result = fs::read_to_string(PROFILES_PATH);
    let json_data = match json_data_result {
        Ok(data) => data,
        Err(_) => {
            println!("Failed to read profiles file");
            process::exit(1);
        },
    };
    let profiles: Data = serde_json::from_str(&json_data).unwrap();
    return profiles.profiles;
}

fn list_profiles() {
    let profiles = get_all_profiles();
    println!("List of available profiles");
    for key in profiles.keys() {
        println!("{}", key);
    }
}

fn switch_profile(profile_name: String) {
    let profiles = get_all_profiles();
    let record_result = profiles.get(profile_name.as_str());
    let record = match record_result {
        None => {
            println!("Profile not found: {}", profile_name);
            process::exit(1);
        },
        Some(record) => record,
    };

    let mut template = fs::read_to_string(TEMPLATE_PATH).unwrap();
    template = template.replace("{Endpoint}", &record.Endpoint);
    template = template.replace("{PublicKey}", &record.PublicKey);

    println!("Stopping wg service");
    Command::new("systemctl").arg("stop").arg("wg-quick@wg0")
        .output()
        .expect("Failed to start wg");

    println!("Writing new configurations");
    let mut file = File::create(CONFIG_FILE_PATH).expect("Failed to open configuration file for writing");
    file.write_all(template.as_bytes()).expect("Failed to write configuration file");
    file.sync_all().expect("Failed to flush changes to configuration file");

    println!("Starting wg service");
    Command::new("systemctl").arg("start").arg("wg-quick@wg0")
        .output()
        .expect("Failed to start wg");
}

fn main()  {
    let args = Cli::parse();

    match args.command {
        Commands::List => list_profiles(),
        Commands::Switch { profile } => switch_profile(profile),
    }
}
