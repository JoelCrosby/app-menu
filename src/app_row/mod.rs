mod imp;

use glib::{BindingFlags, Object};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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

    pub fn bind(&self, app_object: &AppObject) {
        // Get state
        let name_label = self.imp().name_label.get();
        let description_label = self.imp().description_label.get();
        let icon_image = self.imp().icon_image.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `app_object.name` to `app_row.name_label.label`
        let name_label_binding = app_object
            .bind_property("name", &name_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        bindings.push(name_label_binding);

        // Bind `app_object.name` to `app_row.name_label.label`
        let description_label_binding = app_object
            .bind_property("description", &description_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        bindings.push(description_label_binding);

        // Bind `app_object.name` to `app_row.name_label.label`
        let icon_image_binding = app_object
            .bind_property("icon", &icon_image, "gicon")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        // Save binding
        bindings.push(icon_image_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
