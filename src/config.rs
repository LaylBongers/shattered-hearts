use std::path::PathBuf;
use toml::Parser;
use clausewitz_data::file;

pub struct Config {
    pub mod_name: String,
    pub mod_name_friendly: String,
    pub target_path: PathBuf,
    pub game_path: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        info!("Loading config at \"./config/Config.toml\"...");
        let toml = file::read_all_text("./config/Config.toml").unwrap();
        let values = Parser::new(&toml).parse().unwrap();

        let config = Config {
            mod_name: values["mod_name"].as_str().unwrap().into(),
            mod_name_friendly: values["mod_name_friendly"].as_str().unwrap().into(),
            target_path: values["target_path"].as_str().unwrap().into(),
            game_path: values["game_path"].as_str().unwrap().into(),
        };

        config
    }
}
