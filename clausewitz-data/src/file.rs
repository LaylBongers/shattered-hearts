use std::path::Path;
use std::fs::File;
use std::io::{Read, Write, Error};

pub fn read_all_text<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));

    // Trim BOM if needed
    data = data.trim_left_matches('\u{feff}').into();

    Ok(data)
}

pub fn write_all_text<P: AsRef<Path>>(path: P, text: &str) -> Result<(), Error> {
    let mut file = try!(File::create(path));
    try!(file.write_all(text.as_bytes()));
    Ok(())
}

// `read_all_win_1252` and `write_all_win_1252` are no longer needed because the data files are now
// in UTF-8.
