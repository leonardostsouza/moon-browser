#![allow(warnings)]

pub mod gui;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

use std::env::args;

fn main() {
    let application = gtk::Application::new("com.github.basic",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
