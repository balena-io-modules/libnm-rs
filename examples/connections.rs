extern crate nm;

use nm::*;

fn main() {
    let client = Client::new(NONE_CANCELLABLE).unwrap();

    let all_devices = client.get_devices();

    let all_connections: Vec<_> = client
        .get_connections()
        .into_iter()
        .map(|c| c.upcast::<Connection>())
        .collect();

    for device in all_devices {
        println!("============================================================");
        println!(
            "{} [{}]",
            device.get_iface().unwrap(),
            &format!("{}", device.get_device_type()).to_lowercase()[12..],
        );
        println!(
            "{} - {}",
            device.get_vendor().unwrap(),
            device.get_product().unwrap()
        );
        println!("------------------------------------------------------------");

        let device_connections = device.filter_connections(&all_connections);

        for connection in device_connections {
            let setting_connection = connection.get_setting_connection().unwrap();
            println!("{}", setting_connection.get_id().unwrap());
        }
    }
}
