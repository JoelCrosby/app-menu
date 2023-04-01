mod imp;

use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

use crate::app_object::AppObject;

glib::wrapper! {
    pub struct AppRow(ObjectSubclass<imp::AppRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for AppRow {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    // ANCHOR: bind
    pub fn bind(&self, app_object: &AppObject) {
        // Get state
        let content_label = self.imp().content_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `app_object.content` to `app_row.content_label.label`
        let content_label_binding = app_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        bindings.push(content_label_binding);

        // Bind `app_object.completed` to `app_row.content_label.attributes`
        let content_label_binding = app_object
            .bind_property("completed", &content_label, "attributes")
            .flags(BindingFlags::SYNC_CREATE)
            .transform_to(|_, active| {
                let attribute_list = AttrList::new();
                if active {
                    // If "active" is true, content of the label will be strikethrough
                    let attribute = AttrInt::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build();
        // Save binding
        bindings.push(content_label_binding);
    }
    // ANCHOR_END: bind

    // ANCHOR: unbind
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
    // ANCHOR_END: unbind
}
