mod imp;

use glib::Object;
use gtk::{gio::Icon, glib, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct AppObject(ObjectSubclass<imp::AppObject>);
}

impl AppObject {
    pub fn new(name: String, description: String, icon: Icon) -> Self {
        Object::builder()
            .property("name", name)
            .property("description", description)
            .property("icon", icon)
            .build()
    }

    pub fn search(&self, query: &str) -> bool {
        self.imp().data.borrow().name.contains(&query)
    }
}

#[derive(Debug)]
pub struct AppData {
    pub name: String,
    pub description: String,
    pub icon: Icon,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            icon: Icon::for_string(&"").unwrap(),
        }
    }
}
