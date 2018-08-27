extern crate nm;

use nm::prelude::*;

fn main() {
    let client = nm::Client::new(None).unwrap();
    for device in client.get_devices() {
        println!("{}", device.get_iface().unwrap());
    }
}
