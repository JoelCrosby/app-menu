mod app_object;
mod app_row;
mod desktop_entry;
mod reader;
mod window;

extern crate glob;
extern crate ini;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("composite_templates.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("org.gtk_rs.app_menu")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
