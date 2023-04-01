mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct AppObject(ObjectSubclass<imp::AppObject>);
}

impl AppObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct AppData {
    pub completed: bool,
    pub content: String,
}
