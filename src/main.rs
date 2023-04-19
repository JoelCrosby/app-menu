#[macro_use]
extern crate log;

mod app_data;
mod icon;
mod reader;

use env_logger::{Builder, Env};
use gtk::glib::{clone, MainContext};
use gtk::{glib, Application, Box, FlowBox, Image, Label, ScrolledWindow};
use gtk::{prelude::*, Window};

use crate::app_data::AppData;

fn main() -> glib::ExitCode {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    Builder::from_env(env)
        .format_level(true)
        .format_timestamp_millis()
        .init();

    let app = Application::builder()
        .application_id("org.gtk_rs.app_menu")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let flow_box = FlowBox::builder()
        .selection_mode(gtk::SelectionMode::Browse)
        .activate_on_single_click(true)
        .build();

    let scroller = ScrolledWindow::builder()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&flow_box)
        .build();

    let window = Window::builder()
        .application(app)
        .width_request(640)
        .height_request(780)
        .title("App Menu")
        .resizable(false)
        .decorated(false)
        .child(&scroller)
        .build();

    window.present();

    debug!("present started");

    let main_context = MainContext::default();

    main_context.spawn_local(clone!(@weak flow_box => async move {
        debug!("start app iter");

        let apps = match reader::read_fd() {
            Ok(apps) => apps,
            Err(_) => Vec::<AppData>::new(),
        };

        debug!("finished app iter");

        for app in apps {
            let row = build_row(&app);
            flow_box.append(&row);
        }
    }));

    debug!("present finished");
}

fn build_row(app: &AppData) -> Box {
    let row = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .build();

    let img = Image::builder()
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .pixel_size(32)
        .build();

    if app.icon.starts_with("/") {
        img.set_file(Some(app.icon.as_str()));
    } else {
        img.set_icon_name(Some(app.icon.as_str()));
    }

    let text_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let name = Label::builder()
        .halign(gtk::Align::Start)
        .css_classes(vec!["title-4"])
        .build();

    let description = Label::builder()
        .halign(gtk::Align::Start)
        .css_classes(vec!["body"])
        .wrap(true)
        .build();

    name.set_text(&app.name);
    description.set_text(&app.description);

    text_box.append(&name);
    text_box.append(&description);

    row.append(&img);
    row.append(&text_box);

    row
}
