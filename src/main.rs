#[macro_use] extern crate log;
extern crate log4rs;
extern crate toml;
extern crate clausewitz_data;
extern crate clausewitz_game_hoi4;

mod config;

use clausewitz_game_hoi4::{CwGameHoi4, Hoi4Mod};
use config::Config;

struct TagGenerator {
    next_tag: i32
}

impl TagGenerator {
    fn new() -> Self {
        TagGenerator {
            next_tag: 0
        }
    }

    fn get_tag_for_num(num: i32) -> String {
        let mut b = [b'A'; 3];

        b[0] += (num / (26*26)) as u8;
        b[1] += ((num % (26*26)) / 26) as u8;
        b[2] += (num % 26) as u8;

        ::std::str::from_utf8(&b).unwrap().to_string()
    }

    fn next(&mut self, game: &CwGameHoi4) -> String {
        loop {
            // Get the next tag and increment
            let tag = Self::get_tag_for_num(self.next_tag);
            self.next_tag += 1;

            // Make sure it's not one of these special cases
            if tag == "AUX" || tag == "CON" || tag == "AND" {
                continue;
            }

            // Make sure it's not already in use
            if game.countries().iter().any(|v| v.tag() == &tag) {
                continue;
            }

            // It's valid, return it
            return tag;
        }
    }
}

fn main() {
    // Load in configuration
    log4rs::init_file("config/Log4rs.toml", Default::default()).unwrap();
    let config = Config::load();

    // Load in the game data
    let game = CwGameHoi4::at(&config.game_path);

    // Set up the mod file
    let mut modif = Hoi4Mod::new(&config.mod_name, &config.mod_name_friendly, "1.0.1");
    modif.add_tag("Alternative History");

    // Go over all provinces
    let mut tags = TagGenerator::new();
    for state in game.states().iter() {
        info!("Generating country for state \"{}\"...", state.name());

        // Get a copy of the old country
        let mut country = game.country_for_tag(state.country_tag()).unwrap().clone();

        // Get a tag for this country
        country.set_tag(tags.next(&game));

    }

    // Export the mod
    modif.export(&config.target_path);
}
