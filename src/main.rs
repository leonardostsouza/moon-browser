#![allow(warnings)]
extern crate moon_browser as mb;
use mb::gio::prelude::*;
use mb::gtk::prelude::*;

use mb::formality_document::document::*;
use mb::gui;

use std::env::args;

fn main() {
    let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
