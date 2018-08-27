extern crate nm;

use nm::ClientExt;

fn main() {
    let client = nm::Client::new(None).unwrap();
    let connectivity = client.get_connectivity();
    println!("Connectivity: {:?}", connectivity);
}
