#![allow(warnings)]

pub mod gui;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

extern crate jsonrpc_core; // Maybe not necessary
use jsonrpc_core::*;

extern crate reqwest;

use std::env::args;

fn main() {
    let client = reqwest::Client::new();

    let get_req_url = "https://api.infura.io/v1/jsonrpc/ropsten/eth_blockNumber";
    let post_req_url = "https://ropsten.infura.io/";
	let post_json_rpc = r#"{"jsonrpc":"2.0","method":"eth_blockNumber","params": [],"id":1}"#;

    // Infura GET test
    let mut res = client.get(get_req_url).send().expect("Infura GET test failed");
    println!("{}", res.text().unwrap());

    // Infura POST test
    res = client.post(post_req_url)
        .body(post_json_rpc)
        .send().expect("Infura POST test failed");
    println!("{}", res.text().unwrap());

    /*let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());*/
}
