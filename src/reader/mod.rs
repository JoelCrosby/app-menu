mod reader_error;

use anyhow::Result;
use glob::glob;
use ini::Ini;
use reader_error::ReaderError;
use std::path::PathBuf;

use crate::app_object::AppData;
use crate::util::icon;

pub fn read() -> Result<Vec<AppData>> {
    let xdg_home_apps = dirs::home_dir()
        .ok_or(ReaderError::new("failed to get home dir"))?
        .join(".local/share/applications/**/*.desktop")
        .display()
        .to_string();

    let patterns = vec![
        xdg_home_apps.as_str(),
        "/usr/share/applications/**/*.desktop",
    ];

    let mut entries = Vec::<AppData>::new();

    for pattern in patterns {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => match parse(path) {
                    Ok(e) => entries.push(e),
                    Err(_) => (),
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }

    Ok(entries)
}

fn parse(path: PathBuf) -> Result<AppData> {
    let entry = Ini::load_from_file(path)?;

    let section = entry
        .section(Some("Desktop Entry"))
        .ok_or(ReaderError::new("failed to get section"))?;

    let no_display = section
        .get("NoDisplay")
        .map_or(false, |v| v.to_string() == "true");

    if no_display == true {
        return Err(ReaderError::new("entry contained true value for no_display").into());
    }

    let name = section.get("Name").map_or(String::new(), |v| v.to_string());
    let description = section
        .get("Comment")
        .map_or(String::new(), |v| v.to_string());
    let icon_name = section.get("Icon").map_or(String::new(), |v| v.to_string());

    let icon = icon::get_icon_image(icon_name.as_str()).unwrap();

    Ok(AppData {
        name,
        description,
        icon,
    })
}
