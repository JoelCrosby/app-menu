use std::cell::RefCell;
use std::rc::Rc;

use glib::{ParamSpec, ParamSpecString, Value};
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
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("description").build(),
                ParamSpecString::builder("icon").build(),
                ParamSpecString::builder("exec").build(),
                ParamSpecString::builder("seachindex").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().name = input_value;
            }
            "description" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().description = input_value;
            }
            "icon" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().icon = input_value;
            }
            "exec" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().exec = input_value;
            }
            "seachindex" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().seachindex = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.data.borrow().name.to_value(),
            "description" => self.data.borrow().description.to_value(),
            "icon" => self.data.borrow().icon.to_value(),
            "seachindex" => self.data.borrow().seachindex.to_value(),
            _ => unimplemented!(),
        }
    }
}
