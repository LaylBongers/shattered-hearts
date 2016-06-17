use std::path::PathBuf;
use std::fs;
use clausewitz_data::{file, CwTable, CwValue};
use ::{Hoi4Country, Hoi4State, Hoi4Units};

pub struct Hoi4Mod {
    name: String,
    friendly_name: String,
    game_version: String,
    tags: Vec<String>,

    countries: Vec<Hoi4Country>,
    states: Vec<Hoi4State>,
    units: Vec<Hoi4Units>,
}

impl Hoi4Mod {
    pub fn new(name: &str, friendly_name: &str, game_version: &str) -> Self {
        Hoi4Mod {
            name: name.into(),
            friendly_name: friendly_name.into(),
            game_version: game_version.into(),
            tags: Vec::new(),

            countries: Vec::new(),
            states: Vec::new(),
            units: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.into());
    }

    pub fn add_country(&mut self, country: Hoi4Country) {
        self.countries.push(country);
    }

    pub fn add_state(&mut self, state: Hoi4State) {
        self.states.push(state);
    }

    pub fn add_units(&mut self, units: Hoi4Units) {
        self.units.push(units);
    }

    pub fn export(&self, path: &PathBuf) {
        info!("Exporting mod to \"{}\"...", path.display());

        // Make sure the output dir exists
        if !path.exists() {
            panic!("Output path does not exist!");
        }

        // Get the actual directory we export to
        let mut dir = path.clone();
        dir.push(&self.name);

        // Delete the old mod directory if it's already there and create a new mod folder to export to
        if dir.exists() {
            warn!("Directory already exists, deleting stale...");
            fs::remove_dir_all(&dir).unwrap();
        }
        fs::create_dir_all(&dir).unwrap();

        // Create the .mod file
        self.export_modfile(path);

        // Export the data
        self.export_countries(&dir);
        self.export_states(&dir);
        self.export_units(&dir);
    }

    fn export_modfile(&self, path: &PathBuf) {
        info!("Generating .mod file...");

        // Find the actual location of the mod file
        let mut modfile = path.clone();
        modfile.push(format!("{}.mod", &self.name));

        // Generate the data
        let mut dotmod = CwTable::new();
        dotmod.set("name", (&self.friendly_name).into());
        dotmod.set("path", format!("mod/{}", self.name).into());
        dotmod.set("tags", CwValue::Array(self.tags.iter().map(|t| t.into()).collect()));
        dotmod.set("supported_version", (&self.game_version).into());

        // Write the data
        let dotmod_str = dotmod.serialize();
        file::write_all_text(modfile, &dotmod_str, false).unwrap();
    }

    fn export_countries(&self, path: &PathBuf) {
        info!("Exporting countries...");

        // First write the tag-to-country mapping
        let mut country_tags = path.clone();
        country_tags.push("common/country_tags");
        fs::create_dir_all(&country_tags).unwrap();
        country_tags.push("countries.txt");
        file::write_all_text(country_tags, &self.generate_country_tags_table().serialize(), false).unwrap();

        // Get the common and history roots
        let mut common_file_root = path.clone();
        common_file_root.push("common/countries");
        fs::create_dir_all(&common_file_root).unwrap();

        let mut history_file_root = path.clone();
        history_file_root.push("history/countries");
        fs::create_dir_all(&history_file_root).unwrap();

        // Write the actual country common and history files
        for country in &self.countries {
            let mut common_file = common_file_root.clone();
            common_file.push(format!("{}.txt", country.name()));
            file::write_all_text(common_file, &country.common_table().serialize(), false).unwrap();

            let mut history_file = history_file_root.clone();
            history_file.push(format!("{} - {}.txt", country.tag(), country.name()));
            file::write_all_text(history_file, &country.history_table().serialize(), true).unwrap();
        }
    }

    fn generate_country_tags_table(&self) -> CwTable {
        let mut table = CwTable::new();
        for country in &self.countries {
            table.set(country.tag(), format!("countries/{}.txt", country.name()).into());
        }
        table
    }

    fn export_states(&self, path: &PathBuf) {
        info!("Exporting states...");

        let mut state_file_root = path.clone();
        state_file_root.push("history/states");
        fs::create_dir_all(&state_file_root).unwrap();

        for state in &self.states {
            let mut state_file = state_file_root.clone();
            state_file.push(state.file_name());
            file::write_all_text(state_file, &state.data().serialize(), false).unwrap();
        }
    }

    fn export_units(&self, path: &PathBuf) {
        info!("Exporting units...");

        let mut units_file_root = path.clone();
        units_file_root.push("history/units");
        fs::create_dir_all(&units_file_root).unwrap();

        for units in &self.units {
            let mut units_file = units_file_root.clone();
            units_file.push(format!("{}.txt", units.id()));
            file::write_all_text(units_file, &units.data().serialize(), false).unwrap();
        }
    }
}
