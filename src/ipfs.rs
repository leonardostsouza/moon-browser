// ipfs.rs
// Abstraction Layer for interacting with IPFS API

extern crate ipfsapi;
use self::ipfsapi::IpfsApi;

const server : &str = "ipfs.infura.io";
const port : u16 = 5001;
/*static api : IpfsApi = IpfsApi{
    server: server,
    port: port
};*/

pub fn block_get(hash: &str) {
    let api = IpfsApi::new(server, port);

    let bytes = match api.block_get(&hash){
            Ok(raw_data) => raw_data,
            Err(error) => {
                return
            }
    };
    let data =  String::from_utf8(bytes.collect())
                .expect("Unable read data from IPFS block as string");
    data

}

// TODO implement block_put()
pub fn block_put/*<D : 'static + Read + Send>*/(/*data: D*/) {
    /*const method : &str = "api/v0/block/put";
    let url = [ipfs_server, method].join("");
    println!("=====> block_put -- url = {}", url);

    let client = reqwest::Client::new();
    let mut res = client.post(url.as_str())
                .body("O rato roeu a roupa do rei de Roma")
                .send().unwrap();
    println!("=====> block_put -- res = {:?}", res);*/
}
