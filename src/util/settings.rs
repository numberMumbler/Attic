extern crate serde_json;

use std::fs::File;
use std::io::Read;

static CONFIG_FILE: &'static str = "Settings.json";

#[derive(Deserialize)]
pub struct Settings {
    pub access_token: String,
}

pub fn get_settings() -> Settings {
    let mut file = File::open(CONFIG_FILE.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let settings: Settings = serde_json::from_str(&data).unwrap();
    return settings;
}
