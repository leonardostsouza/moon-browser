#![allow(warnings)]

pub mod gui;
//pub mod ipfs;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

/*extern crate web3;
use web3::futures::Future;
use web3::types::Address;
*/
//extern crate reqwest;

/*extern crate ipfsapi;
use self::ipfsapi::IpfsApi;*/

use std::env::args;

fn main() {
    // IPFS Mod test
/*    let hash = "Qmb3qWYf462ypcjP9KRLAozEM7xSo4aDNNJchFZfDSyQSE";
    println!("{:?}", ipfs::block_get(hash));*/



    let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
