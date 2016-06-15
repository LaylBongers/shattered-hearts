#[macro_use] extern crate log;
extern crate clausewitz_data;

use std::path::{Path, PathBuf};
use clausewitz_data::{file, CwTable};

pub struct Hoi4State {
    data: CwTable,
}

impl Hoi4State {
    pub fn load(path: &Path) -> Self {
        let text = file::read_all_text(path).unwrap();
        let file_data = CwTable::parse(&text);

        Hoi4State {
            data: file_data
        }
    }

    pub fn name(&self) -> &String {
        let state = self.data.get("state").unwrap().as_table().unwrap();
        state.get("name").unwrap().as_string().unwrap()
    }
}

pub struct CwGameHoi4 {
    states: Vec<Hoi4State>
}

impl CwGameHoi4 {
    pub fn at(path: &PathBuf) -> Self {
        let states = Self::load_states(path);

        CwGameHoi4 {
            states: states
        }
    }

    fn load_states(path: &PathBuf) -> Vec<Hoi4State> {
        // Get the root of the states directory
        let mut dir = path.clone();
        dir.push("history");
        dir.push("states");
        assert!(dir.is_dir(), "\"{}\" is not an existing directory", dir.display());

        // Go over all the files in that directory
        let mut states = Vec::new();
        for file_r in dir.read_dir().unwrap() {
            let file = file_r.unwrap();
            debug!("Loading {:?}...", file.file_name());

            // Load in the state
            states.push(Hoi4State::load(&file.path()));
        }

        states
    }

    pub fn states(&self) -> &Vec<Hoi4State> {
        &self.states
    }
}
