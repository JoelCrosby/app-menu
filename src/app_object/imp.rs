use std::cell::RefCell;
use std::rc::Rc;

use glib::{ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

use super::AppData;

#[derive(Default)]
pub struct AppObject {
    pub data: Rc<RefCell<AppData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppObject {
    const NAME: &'static str = "AppObject";
    type Type = super::AppObject;
}

impl ObjectImpl for AppObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::builder("completed").build(),
                ParamSpecString::builder("content").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "completed" => {
                let input_value = value.get().expect("The value needs to be of type `bool`.");
                self.data.borrow_mut().completed = input_value;
            }
            "content" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().content = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "completed" => self.data.borrow().completed.to_value(),
            "content" => self.data.borrow().content.to_value(),
            _ => unimplemented!(),
        }
    }
}
