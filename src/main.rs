#![allow(warnings)]
extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate gdk;
extern crate glib;
extern crate cairo;

extern crate serde_json;

#[macro_use] extern crate lazy_static;

extern crate formality_document;
use formality_document::document::*;
use formality_document::formality::term::{Defs, Term};

mod math;
mod ipfs;
mod app_api;
mod gui;

use app_api::App;
use std::env::args;
use std::sync::Mutex;

lazy_static! {
    static ref APP: Mutex<App> = Mutex::new(App::blank());
}

fn main() {
    let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");


    application.connect_startup(|gtk_app| {
        gui::build_ui(gtk_app, &APP, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
