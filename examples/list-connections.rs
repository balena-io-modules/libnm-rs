extern crate nm;

mod common;

use common::*;
use nm::*;

fn main() {
    let client = Client::new(NONE_CANCELLABLE).unwrap();

    let all_devices = client.devices();

    let all_connections: Vec<_> = client
        .connections()
        .into_iter()
        .map(|c| c.upcast::<Connection>())
        .collect();

    for device in all_devices {
        println!("======================================================================");
        print_device_info(&device);

        println!("");
        let device_connections = device.filter_connections(&all_connections);

        if device_connections.is_empty() {
            println!("-")
        }

        for connection in device_connections {
            if let Some(setting_connection) = connection.setting_connection() {
                if let Some(id) = setting_connection.id() {
                    if let Some(uuid) = setting_connection.uuid() {
                        println!("{:31} [{}]", id.as_str(), uuid);
                    }
                }
            }
        }
    }
}
