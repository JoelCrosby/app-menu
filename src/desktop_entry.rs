use std::fmt::Debug;

pub struct DesktopEntry {
    pub name: String,
    pub path: String,
}

impl Debug for DesktopEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DesktopEntry")
            .field("name", &self.name)
            .field("path", &self.path)
            .finish()
    }
}
