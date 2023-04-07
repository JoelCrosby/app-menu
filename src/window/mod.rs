mod imp;

use anyhow::Result;
use glib::Object;
use gtk::glib::clone;
use gtk::prelude::{Cast, CastNone, StaticType};
use gtk::subclass::prelude::*;
use gtk::traits::EditableExt;
use gtk::{
    gio, glib, Adjustment, Application, CustomFilter, FilterListModel, ListItem,
    SignalListItemFactory, SingleSelection,
};

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
            .to_owned()
            .expect("could not access filter")
    }

    fn setup_apps(&self) {
        debug!(target: "app_events", "setup_apps");

        let model = gio::ListStore::new(AppObject::static_type());

        // Get state and set model
        self.imp().apps.replace(Some(model));
        let filter_model = FilterListModel::new(Some(self.apps()), self.filter("".to_string()));
        let selection_model = SingleSelection::new(Some(filter_model.clone()));
        self.imp().apps_list.set_model(Some(&selection_model));

        self.imp()
            .entry
            .connect_changed(clone!(@weak self as window => move |entry| {
                let text = entry.text();
                let query = text.to_ascii_lowercase();
                let filter = window.filter(query.to_string());

                filter_model.set_filter(filter.as_ref());

                let adj = Adjustment::builder().value(0.0).build();
                window.imp().scoll_window.set_vadjustment(Some(&adj));
            }));
    }

    fn new_app(&self, data: &AppData) {
        // Add new app to model
        let app = AppObject::new(&data);
        self.apps().append(&app);
    }

    fn setup_factory(&self) {
        debug!(target: "app_events", "setup_factory");

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
        debug!(target: "app_events", "poulate");

        let entries = reader::read_fd()?;

        for entry in &entries {
            self.new_app(entry);
        }

        debug!(target: "performance", "poulate complete");

        Ok(())
    }

    fn filter(&self, query: String) -> Option<CustomFilter> {
        // Create custom filters
        let query_filter = CustomFilter::new(move |obj| {
            if query.is_empty() {
                return true;
            }

            // Get `AppObject` from `glib::Object`
            let app_object = obj
                .downcast_ref::<AppObject>()
                .expect("The object needs to be of type `AppObject`.");

            app_object.search(&query.as_str())
        });

        Some(query_filter)
    }
}
