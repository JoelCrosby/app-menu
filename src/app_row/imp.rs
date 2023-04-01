use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/app_menu/app_row.ui")]
pub struct AppRow {
    #[template_child]
    pub content_label: TemplateChild<Label>,

    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppRow {
    const NAME: &'static str = "AppRow";
    type Type = super::AppRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AppRow {}

impl WidgetImpl for AppRow {}

impl BoxImpl for AppRow {}
