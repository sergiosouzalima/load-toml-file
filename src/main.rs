use indoc::indoc;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use toml;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    source_file: String,
    target_file: String,
    source_file_skip_rows: u16,
    email_validation: bool,
    filter_for: Vec<String>,
}

fn deserialize_toml(file_contents: &str) -> Result<Data, Box<dyn Error>> {
    toml::from_str(file_contents).map_err(|e| e.into())
}

fn main() {
    println!(
        "\nChecking if config file {} is available....",
        CONFIG_FILE_NAME
    );
    if !Path::new(CONFIG_FILE_NAME).exists() {
        let content = indoc! {r#"
            [config]
            source_file = "Connections.csv"
            target_file = "result_connections.csv"
            source_file_skip_rows = 3
            email_validation = true
            filter_for = ["HR", "RH", "Human Resources", "Recursos Humanos", "Talent", "Talento", "Talentos", 
                          "Recruiter", "Recrutador", "Recrutadora", "Gente", "People", "Attraction", 
                          "Atração", "Benefit", "Benefits", "Benefício", "Benefícios"]
        "#};
        println!("Creating config file {}....", CONFIG_FILE_NAME);
        fs::write(CONFIG_FILE_NAME, content).expect("Failed to create config file");
    }
    let contents = fs::read_to_string(CONFIG_FILE_NAME).expect("Failed to open config file");

    let data = match deserialize_toml(&contents) {
        Ok(d) => d,
        Err(e) => {
            println!("Error deserializing {}: {}", CONFIG_FILE_NAME, e);
            return;
        }
    };

    println!("{}", data.config.source_file);
    println!("{}", data.config.target_file);
    println!("{}", data.config.source_file_skip_rows);
    println!("{}", data.config.email_validation);
    println!("{:#?}", data.config.filter_for);    
}
