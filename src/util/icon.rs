use gtk::{gio::Icon, IconLookupFlags, IconTheme, TextDirection};

pub fn get_icon_image(icon: &str) -> Option<Icon> {
    let theme = IconTheme::new();
    let icon_info = theme.lookup_icon(
        icon,
        &[],
        48,
        1,
        TextDirection::None,
        IconLookupFlags::PRELOAD,
    );

    let icon_name = match icon_info.icon_name() {
        Some(i) => i.display().to_string(),
        None => return None,
    };

    Icon::for_string(icon_name.as_str()).ok()
}
