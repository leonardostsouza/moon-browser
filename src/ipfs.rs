// ipfs.rs
// Abstraction Layer for interacting with IPFS API

extern crate ipfsapi;
use self::ipfsapi::IpfsApi;

const SERVER : &str = "ipfs.infura.io";
const PORT : u16 = 5001;
// TODO: Instead of creating a new interface at every function call, make it a
// module global instead
// eg.: let api = IpfsApi::new(server, port);


pub fn block_get(hash: &str) -> String {
    let api = IpfsApi::new(SERVER, PORT);

    // TODO: Implement proper Error handling
    let bytes = api.block_get(&hash).unwrap();
    let data =  String::from_utf8(bytes.collect())
                .expect("Unable read data from IPFS block as string");
    data
}

// TODO implement block_put()
pub fn block_put(data: &'static [u8]/*&'static str*/) -> String {
    let api = IpfsApi::new(SERVER, PORT);
    // WARNING: The type "&'static [u8]" of the data variable was a workaround
    // needed because the function "block_put"
    // from IpfsApi only accepts it's argument if
    // it has 'static lifetime. The best aproach, however, would be to
    // modify IpfsApi to remove this limitation if possible.
    //
    // TODO: Modify IpfsApi to remove the need of 'static lifetime
    // from function block_put() if possible
    let hash = match api.block_put(data/*.as_bytes()*/) {
        Ok(block_hash) => block_hash,
        Err(_) => {
            String::from("ERROR: Unable to put IPFS block")
        }
    };
    hash
}
