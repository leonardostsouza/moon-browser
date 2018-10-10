#![allow(warnings)]

pub mod gui;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

use std::env::args;

fn main() {
    let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
