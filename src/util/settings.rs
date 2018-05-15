extern crate serde_json;

use std::fs::File;
use std::io::Read;

const CONFIG_FILE: &str = "Settings.json";

#[derive(Deserialize)]
pub struct Settings {
    pub access_token: String,
}

impl Settings {
    pub fn new() -> Settings {
        Settings::get_settings()
    }

    fn get_settings() -> Settings {
        let mut file = File::open(CONFIG_FILE).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let settings: Settings = serde_json::from_str(&data).unwrap();
        return settings;
    }
}
