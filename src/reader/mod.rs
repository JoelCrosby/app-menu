use anyhow::Result;
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use gtk::IconTheme;
use std::borrow::Cow;
use std::fs;

use crate::app_object::AppData;
use crate::util::icon;

fn get(value: Option<Cow<str>>) -> String {
    value.map(|e| e.to_string()).unwrap_or_default()
}

pub fn read_fd() -> Result<Vec<AppData>> {
    let icon_theme = IconTheme::new();

    let mut entries = Vec::<AppData>::new();

    for path in Iter::new(default_paths()) {
        let bytes = match fs::read_to_string(&path) {
            Ok(res) => res,
            Err(_) => continue,
        };

        let entry = match DesktopEntry::decode(&path, &bytes) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let no_display = entry.no_display();

        if no_display {
            continue;
        }

        let e_type = entry.type_().unwrap_or_default();

        if e_type.is_empty() {
            continue;
        }

        let name = get(entry.name(None));

        if name.is_empty() {
            continue;
        }

        let description = get(entry.comment(None));
        let icon_name = entry.icon().unwrap_or_default();
        let exec = entry.exec().unwrap_or_default().to_string();

        if exec.is_empty() {
            continue;
        }

        let icon = match icon::get_icon_image(icon_name, &icon_theme) {
            Some(icon) => icon,
            None => String::new(),
        };

        entries.push(AppData {
            name,
            description,
            icon,
            exec,
        })
    }

    Ok(entries)
}
