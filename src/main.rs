#![allow(warnings)]

pub mod gui;
pub mod ipfs;

extern crate gio;
use gio::prelude::*;

extern crate gtk;
use gtk::prelude::*;

extern crate web3;
use web3::futures::Future;
use web3::types::Address;

extern crate reqwest;

extern crate ipfsapi;
use self::ipfsapi::IpfsApi;

use std::env::args;

fn main() {
    // Ethereum Network Test
    /*let (_eloop, http) = web3::transports::Http::new("https://ropsten.infura.io/").unwrap();
    let web3 = web3::Web3::new(http);
    let bln = web3.eth().block_number().wait().unwrap();
    println!("Latest Block Number: {:?}", bln);

    let acc = web3.eth().accounts().wait().unwrap();
    println!("Acounts: {:?}", acc);

    let vec: &[u8] = b"0xcEBD3825A1995A30EA8Bd0666E959f0c8bcEfCF3";
    let mut addr: Address = Address::new();
    addr.clone_from_slice(vec);
    //let addr: Address = Address::random();
    let balance = web3.eth().balance(addr, None).wait().unwrap();
    println!("Vec: {:?}\nAddress: {:x?}\nBalance: {:?}", vec, addr, balance);*/

    // IPFS test
    //let url = "https://ipfs.infura.io:5001/api/v0/block/get?arg=";
    let hash = "QmfM2r8seH2GiRaC4esTjeraXEachRt8ZsSeGaWTPLyMoG";
    //let body = reqwest::get([url, hash].join("").as_str());
    //println!("Body:\n{:?}", body.unwrap().text());

    // IPFS Mod test
    println!("{:?}", ipfs::block_get(hash));
    //ipfs::block_get(hash);



    println!("1");
    let api = IpfsApi::new("ipfs.infura.io", 5001);
    //let hash = "QmfM2r8seH2GiRaC4esTjeraXEachRt8ZsSeGaWTPLyMoG";
    println!("2");
    let bytes = match api.block_get(&hash){
            Ok(raw_data) => raw_data,
            Err(error) => {
                return
            }
    };
    println!("3");
    let data =  String::from_utf8(bytes.collect())
                .expect("Unable read data from IPFS block as string");
    println!("4");
    println!("{}", data);
    println!("5");


    /*let application = gtk::Application::new("org.ethereum.rs.moon",
                                        gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app, 800, 600);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());*/
}
