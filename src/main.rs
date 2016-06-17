#[macro_use] extern crate log;
extern crate log4rs;
extern crate toml;
extern crate rand;
extern crate clausewitz_data;
extern crate clausewitz_game_hoi4;

mod config;

use rand::{Rng, StdRng};
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
    info!("Loading Hearts of Iron 4 data...");
    let game = CwGameHoi4::at(&config.game_path);

    // Set up the mod file
    let mut modif = Hoi4Mod::new(&config.mod_name, &config.mod_name_friendly, "1.0.1");
    modif.add_tag("Alternative History");

    // Go over all states
    let mut tags = TagGenerator::new();
    let mut rng = StdRng::new().unwrap();
    let mut next_id = 1000;
    for state in game.states().iter() {
        info!("Generating country for state \"{}\"...", state.name());

        // Copy the country with a new name and tag for this state
        let mut country = game.country_for_tag(state.owner()).unwrap().clone();
        country.set_tag(tags.next(&game));
        country.set_name(state.name().clone());
        country.set_color(rng.gen(), rng.gen(), rng.gen());
        country.set_capital(state.id().clone());

        // Replace IDs in the country so they don't conflict
        country.replace_ids(|| {next_id+=1; next_id-1});

        // Give the country -25% time needed to justify war goals, prevents years of waiting
        country.add_idea("shattered_hearts_fractured_state".into());

        // Copy the state so we can assign ownership
        let mut modif_state = state.clone();
        modif_state.set_owner(country.tag().clone());
        modif_state.set_controller(country.tag().clone());
        modif_state.add_core(country.tag().clone());

        // Copy the units layout so we can make customize it for this country
        let mut units = game.units_for_id(country.units()).unwrap().clone();
        units.set_id(format!("{}_1936", country.tag()));
        country.set_units(units.id().clone());

        // Elimiate all starting units
        units.clear();

        // Add the data we need in the mod
        modif.add_country(country);
        modif.add_state(modif_state);
        modif.add_units(units);
    }

    // Export the mod
    modif.export(&config.target_path);
}
