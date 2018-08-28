extern crate nm;

use nm::prelude::*;

fn main() {
    let client = nm::Client::new(None).unwrap();
    let connectivity = client.get_connectivity();
    println!("Connectivity: {:?}", connectivity);

    let connectivity_check_enabled: bool = client
        .get_property("connectivity-check-enabled")
        .unwrap()
        .get()
        .unwrap();
    println!(
        "Connectivity Check Enabled: {:?}",
        connectivity_check_enabled
    );
}
