#![allow(warnings)]
extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate gdk;
extern crate glib;
extern crate cairo;

extern crate serde_json;

extern crate formality_document;
use formality_document::document::*;

mod math;
mod ipfs;
mod gui;

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
