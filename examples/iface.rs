extern crate nm;

use nm::prelude::*;

fn main() {
    let client = nm::Client::new(None).unwrap();

    for device in client.get_devices() {
        println!(
            "{}: {}",
            device.get_iface().unwrap(),
            device.downcast::<nm::DeviceWifi>().is_ok()
        );
    }

    println!("==========");

    for active_connection in client.get_active_connections() {
        println!(
            "{}: {}",
            active_connection.get_id().unwrap(),
            active_connection.get_uuid().unwrap()
        );
    }
}
