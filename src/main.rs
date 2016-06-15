#[macro_use] extern crate log;
extern crate log4rs;
extern crate toml;
extern crate clausewitz_data;
extern crate clausewitz_game_hoi4;
extern crate clausewitz_mod;

mod config;

use clausewitz_game_hoi4::CwGameHoi4;
use clausewitz_mod::CwMod;
use config::Config;

fn main() {
    // Load in configuration
    log4rs::init_file("config/Log4rs.toml", Default::default()).unwrap();
    let config = Config::load();

    // Load in the game data
    let game = CwGameHoi4::at(&config.game_path);

    // Set up the mod file
    let mut modif = CwMod::new(&config.mod_name, &config.mod_name_friendly, "1.0.1");
    modif.add_tag("Alternative History");

    // Go over all provinces
    for state in game.states().iter() {
        info!("Generating country for state \"{}\"...", state.name());

        // Get a copy of the old country
        let country = game.country_for_tag(state.country_tag()).clone();
    }

    // Export the mod
    modif.export(&config.target_path);
}
