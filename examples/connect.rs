extern crate nm;

fn main() {
    //    let client = nm::Client::new(None).unwrap();

    let uuid = nm::utils_uuid_generate();

    println!("{}", uuid);
}
