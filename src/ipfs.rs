// ipfs.rs
// Abstraction Layer for interacting with IPFS

extern crate reqwest;

use std::io::Read;

const ipfs_server : &str = "https://ipfs.infura.io:5001/";

pub fn block_get(hash: &str) -> Result<String, reqwest::Error>{
    let method : &str = "api/v0/block/get?arg=";
    let url = [[ipfs_server, method].join(""), hash.to_owned()].join("");
    let body = reqwest::get(url.as_str());
    body.unwrap().text()
}

// TODO implement block_put()
pub fn block_put/*<D : 'static + Read + Send>*/(/*data: D*/) {
    const method : &str = "api/v0/block/put";
    let url = [ipfs_server, method].join("");
    println!("=====> block_put -- url = {}", url);

    let client = reqwest::Client::new();
    let mut res = client.post(url.as_str())
                .body("O rato roeu a roupa do rei de Roma")
                .send().unwrap();
    println!("=====> block_put -- res = {:?}", res);
}
