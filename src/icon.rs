use gtk::{IconLookupFlags, IconTheme, TextDirection};

pub fn get_icon_image(icon: &str, theme: &IconTheme) -> Option<String> {
    if icon.contains('/') {
        return Some(icon.to_string());
    }

    let icon_info = theme.lookup_icon(
        icon,
        &[],
        0,
        1,
        TextDirection::None,
        IconLookupFlags::PRELOAD,
    );

    let icon_name = match icon_info.icon_name() {
        Some(i) => i.display().to_string(),
        None => String::new(),
    };

    Some(icon_name)
}
