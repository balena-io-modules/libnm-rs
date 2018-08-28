extern crate nm;

fn main() {
    let client = nm::Client::new(None).unwrap();
    let connectivity = client.get_connectivity();
    println!("Connectivity: {:?}", connectivity);

    let connectivity_check_enabled = client.get_property_connectivity_check_enabled();
    println!(
        "Connectivity check enabled: {:?}",
        connectivity_check_enabled
    );
}
