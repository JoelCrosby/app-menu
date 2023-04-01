mod reader_error;

use anyhow::Result;
use glob::glob;
use ini::Ini;
use reader_error::ReaderError;
use std::path::PathBuf;

pub fn read() -> Result<Vec<String>> {
    let pattern = &"/home/joel/.local/share/applications/**/*.desktop";
    let mut entries = Vec::<String>::new();

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => entries.push(parse(path)?),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(entries)
}

fn parse(path: PathBuf) -> Result<String> {
    let entry = Ini::load_from_file(path)?;

    let section = entry
        .section(Some("Desktop Entry"))
        .ok_or(ReaderError::new("faile dto get section"))?;

    let name = section
        .get("Name")
        .ok_or(ReaderError::new("failed to read name"))?
        .to_string();

    Ok(name)
}
