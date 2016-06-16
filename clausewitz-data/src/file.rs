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

pub fn write_all_text<P: AsRef<Path>>(path: P, text: &str, add_bom: bool) -> Result<(), Error> {
    let mut file = try!(File::create(path));

    // Add BOM if needed
    if add_bom {
        try!(file.write_all("\u{feff}".as_bytes()));
    }

    try!(file.write_all(text.as_bytes()));

    Ok(())
}
