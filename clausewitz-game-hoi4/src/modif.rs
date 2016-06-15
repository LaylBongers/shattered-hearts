use std::path::PathBuf;
use std::fs;
use clausewitz_data::{file, CwTable, CwValue};

pub struct Hoi4Mod {
    name: String,
    friendly_name: String,
    game_version: String,
    tags: Vec<String>,
}

impl Hoi4Mod {
    pub fn new(name: &str, friendly_name: &str, game_version: &str) -> Self {
        Hoi4Mod {
            name: name.into(),
            friendly_name: friendly_name.into(),
            game_version: game_version.into(),
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.into());
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

        // Delete the old mod directory if it's already there
        if dir.exists() {
            warn!("Directory already exists, deleting stale...");
            fs::remove_dir_all(&dir).unwrap();
        }

        // Create a new mod folder for us
        fs::create_dir_all(&dir).unwrap();

        // Create the .mod file
        self.export_mod(path);
    }

    fn export_mod(&self, path: &PathBuf) {
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
        file::write_all_text(modfile, &dotmod_str).unwrap();
    }
}
