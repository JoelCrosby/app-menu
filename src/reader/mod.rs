use anyhow::Result;
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use std::borrow::Cow;
use std::fs;
use std::time::Instant;

use crate::app_object::AppData;
use crate::util::icon;

fn get(value: Option<Cow<str>>) -> String {
    value.map(|e| e.to_string()).unwrap_or_default()
}

pub fn read_fd() -> Result<Vec<AppData>> {
    let now = Instant::now();

    let mut entries = Vec::<AppData>::new();

    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
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

                let icon = icon::get_icon_image(icon_name).unwrap();

                let seachindex = format!(
                    "{} {}",
                    name.to_ascii_lowercase(),
                    description.to_ascii_lowercase()
                );

                entries.push(AppData {
                    name,
                    description,
                    icon,
                    seachindex,
                    exec,
                })
            }
        }
    }

    let elapsed = now.elapsed();
    println!("read_fd elapsed: {:.2?}", elapsed);

    Ok(entries)
}
