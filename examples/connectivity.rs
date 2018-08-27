extern crate nm;

fn main() {
    let client = nm::Client::new(None).unwrap();
    let connectivity = client.get_connectivity();
    println!("Connectivity: {:?}", connectivity);
}
