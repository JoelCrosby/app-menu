mod imp;

use anyhow::Result;
use glib::Object;
use gtk::prelude::{Cast, CastNone, StaticType};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, ListItem, SignalListItemFactory, SingleSelection};

use crate::app_object::{AppData, AppObject};
use crate::app_row::AppRow;
use crate::reader;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn apps(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .apps
            .borrow()
            .clone()
            .expect("Could not get current apps.")
    }

    fn setup_apps(&self) {
        let model = gio::ListStore::new(AppObject::static_type());

        // Get state and set model
        self.imp().apps.replace(Some(model));

        // Wrap model with filter and selection and pass it to the list view

        let selection_model = SingleSelection::new(Some(self.apps()));
        self.imp().apps_list.set_model(Some(&selection_model));
    }

    fn new_app(&self, data: AppData) {
        // Add new app to model
        let app = AppObject::new(data.name, data.description, data.icon);
        self.apps().append(&app);
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `AppRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `AppRow`
            let app_row = AppRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&app_row));
        });

        // Tell factory how to bind `AppRow` to a `AppObject`
        factory.connect_bind(move |_, list_item| {
            // Get `AppObject` from `ListItem`
            let app_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<AppObject>()
                .expect("The item has to be an `AppObject`.");

            // Get `AppRow` from `ListItem`
            let app_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<AppRow>()
                .expect("The child has to be a `AppRow`.");

            app_row.bind(&app_object);
        });

        // Tell factory how to unbind `AppRow` from `AppObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `AppRow` from `ListItem`
            let app_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<AppRow>()
                .expect("The child has to be a `AppRow`.");

            app_row.unbind();
        });

        // Set the factory of the list view
        self.imp().apps_list.set_factory(Some(&factory));
    }

    fn poulate(&self) -> Result<()> {
        let entries = reader::read()?;

        for entry in entries {
            self.new_app(entry);
        }

        Ok(())
    }

    fn setup_callbacks(&self) {}
}
