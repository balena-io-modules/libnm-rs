extern crate nm;

use nm::*;

fn main() {
    let client = Client::new(NONE_CANCELLABLE).unwrap();
    let connectivity = client.connectivity();
    let check_enabled = client.connectivity_check_get_enabled();

    println!("Connectivity: {:?}", connectivity);
    println!("Connectivity check enabled: {:?}", check_enabled);
}
