mod imp;

use glib::Object;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct AppObject(ObjectSubclass<imp::AppObject>);
}

impl AppObject {
    pub fn new(data: &AppData) -> Self {
        Object::builder()
            .property("name", &data.name)
            .property("description", &data.description)
            .property("icon", &data.icon)
            .property("exec", &data.exec)
            .property("seachindex", &data.seachindex)
            .build()
    }

    pub fn search(&self, query: &str) -> bool {
        self.imp().data.borrow().seachindex.contains(query)
    }
}

#[derive(Debug)]
pub struct AppData {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub exec: String,
    pub seachindex: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            icon: Default::default(),
            exec: Default::default(),
            seachindex: Default::default(),
        }
    }
}
