#[macro_use]
extern crate log;

mod app_object;
mod app_row;
mod desktop_entry;
mod reader;
mod util;
mod window;

use env_logger::{Builder, Env};
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    Builder::from_env(env)
        .format_level(true)
        .format_timestamp_millis()
        .init();

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
    window.set_decorated(false);
    window.present();
}
