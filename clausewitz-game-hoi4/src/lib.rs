#[macro_use] extern crate log;
extern crate clausewitz_data;

mod modif;

pub use modif::Hoi4Mod;

use std::path::PathBuf;
use clausewitz_data::{file, CwTable};

#[derive(Clone)]
pub struct Hoi4Country {
    tag: String,
    name: String,
    common: CwTable,
    history: CwTable,
}

impl Hoi4Country {
    pub fn load(tag: String, name: String, common: CwTable, history: CwTable) -> Self {
        Hoi4Country {
            tag: tag,
            name: name,
            common: common,
            history: history,
        }
    }

    pub fn tag(&self) -> &String {
        &self.tag
    }

    pub fn set_tag(&mut self, value: String) {
        self.tag = value
    }
}

#[derive(Clone)]
pub struct Hoi4State {
    data: CwTable,
}

impl Hoi4State {
    pub fn load(data: CwTable) -> Self {
        Hoi4State {
            data: data
        }
    }

    fn state_table(&self) -> &CwTable {
        self.data.get("state").unwrap().as_table().unwrap()
    }

    fn history_table(&self) -> &CwTable {
        self.state_table().get("history").unwrap().as_table().unwrap()
    }

    pub fn name(&self) -> &String {
        self.state_table().get("name").unwrap().as_string().unwrap()
    }

    pub fn country_tag(&self) -> &String {
        self.history_table().get("owner").unwrap().as_string().unwrap()
    }
}

pub struct CwGameHoi4 {
    countries: Vec<Hoi4Country>,
    states: Vec<Hoi4State>,
}

impl CwGameHoi4 {
    pub fn at(path: &PathBuf) -> Self {
        // Load in data related to countries
        let country_histories = Self::load_directory(path, "history", "countries");
        let country_commons = Self::load_directory(path, "common", "countries");

        // Load in the country tag mapping
        let mut country_tags_file = path.clone();
        country_tags_file.push("common/country_tags/00_countries.txt");
        let text = file::read_all_text(&country_tags_file).unwrap();
        let country_tags_data = CwTable::parse(&text);
        let country_tags = country_tags_data.values.iter()
            .map(|v| (v.key.clone(), v.value.as_string().unwrap()));

        // Load in the countries
        let mut countries = Vec::new();
        for country_tag in country_tags {
            // When we reach this we're done with the file
            if country_tag.0 == "dynamic_tags" {
                break;
            }

            debug!("Combining common and history for {}...", country_tag.0);

            // Split the target file into sections then merge it back to extract the name
            // This isn't localization related, this is just the name the files use to refer to it
            let file_name_segments: Vec<_> = country_tag.1
                .split(|c| c == '/' || c == ' ' || c == '-' || c == '.')
                .filter(|s| s.len() != 0 && s != &"txt")
                .skip(1)
                .collect();
            let mut segment_iter = file_name_segments.iter();
            let mut country_name = String::new();
            country_name.push_str(segment_iter.next().unwrap());
            for segment in segment_iter {
                country_name.push(' ');
                country_name.push_str(&segment);
            }

            // Find the common file for this country
            let common = country_commons.iter()
                .find(|c| c.0.contains(&country_name)).unwrap().clone();

            // Find the history file associated with this tag
            let history = country_histories.iter()
                .find(|h| h.0.starts_with(&country_tag.0)).unwrap().clone();

            countries.push(Hoi4Country::load(country_tag.0, country_name, common.1, history.1));
        }

        // Load in the states
        let states = Self::load_directory(path, "history", "states").into_iter()
            .map(|t| Hoi4State::load(t.1)).collect();

        // Create the container type holding all the data
        CwGameHoi4 {
            countries: countries,
            states: states,
        }
    }

    fn load_directory(path: &PathBuf, sub1: &str, sub2: &str) -> Vec<(String, CwTable)> {
        // Get the root of the states directory
        let mut dir = path.clone();
        dir.push(sub1);
        dir.push(sub2);
        assert!(dir.is_dir(), "\"{}\" is not an existing directory", dir.display());

        // Go over all the files in that directory
        let mut states = Vec::new();
        for file_r in dir.read_dir().unwrap() {
            let file = file_r.unwrap();
            let file_name = String::from(file.file_name().to_str().unwrap());
            debug!("Loading {}/{}/{}...", sub1, sub2, file_name);

            // Load in the table
            let text = file::read_all_text(&file.path()).unwrap();
            let file_data = CwTable::parse(&text);
            states.push((file_name, file_data));
        }

        states
    }

    pub fn states(&self) -> &Vec<Hoi4State> {
        &self.states
    }

    pub fn countries(&self) -> &Vec<Hoi4Country> {
        &self.countries
    }

    pub fn country_for_tag(&self, tag: &str) -> Option<&Hoi4Country> {
        self.countries.iter().find(|c| c.tag() == tag)
    }
}
