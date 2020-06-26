#![allow(unused_variables)]
#![allow(dead_code)]

extern crate nm;

use nm::*;

use anyhow::{anyhow, Result};
use clap::Clap;
use futures_util::future::pending;

use glib::translate::FromGlib;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    interface: Option<String>,

    #[clap(short, long, default_value = "NMAccessPoint")]
    ssid: String,

    #[clap(short, long, default_value = "00000000")]
    password: String,

    #[clap(short, long, default_value = "192.168.42.1")]
    address: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let context = glib::MainContext::default();
    let loop_ = glib::MainLoop::new(Some(&context), false);

    context.push_thread_default();

    context.spawn_local(run(opts, loop_.clone()));

    loop_.run();

    context.pop_thread_default();

    Ok(())
}

async fn run(opts: Opts, loop_: glib::MainLoop) {
    let client = Client::new_async_future().await.unwrap();

    let device = get_device(&client, opts.interface.as_deref()).unwrap();

    print_device_info(&device);

    let connection = create_connection(device.get_iface().as_deref(), &opts);

    let active_connection = client
        .add_and_activate_connection_async_future(Some(&connection), &device, None)
        .await
        .unwrap();

    active_connection.connect_state_changed(move |active_connection, state, _| {
        let state = ActiveConnectionState::from_glib(state as _);
        println!("Active connection state: {:?}", state);

        match state {
            ActiveConnectionState::Activated => {
                println!("Successfully activated");
                loop_.quit();
            }
            ActiveConnectionState::Deactivated => {
                println!("Connection NOT activated!");
                loop_.quit();
            }
            _ => {}
        }
    });

    pending().await
}

fn get_device(client: &Client, interface: Option<&str>) -> Result<Device> {
    if let Some(interface) = interface {
        get_exact_device(client, interface)
    } else {
        find_wifi_device(client)
    }
}

fn get_exact_device(client: &Client, interface: &str) -> Result<Device> {
    if let Some(device) = client.get_device_by_iface(interface) {
        if device.get_device_type() != DeviceType::Wifi {
            Err(anyhow!("Not a Wi-Fi device: {}", interface))
        } else {
            Ok(device)
        }
    } else {
        Err(anyhow!("Interface not found: {}", interface))
    }
}

fn find_wifi_device(client: &Client) -> Result<Device> {
    let mut devices = client.get_devices();

    let position = devices
        .iter()
        .position(|d| d.get_device_type() == DeviceType::Wifi);

    if let Some(i) = position {
        Ok(devices.swap_remove(i))
    } else {
        Err(anyhow!("Cannot find a Wi-Fi device"))
    }
}

fn print_device_info(device: &Device) {
    if let Some(interface) = device.get_iface() {
        if let Some(description) = device.get_description() {
            println!("Use device: {} / {}", interface, description);
        } else {
            println!("Use device: {}", interface);
        }
    }
}

fn create_connection(interface: Option<&str>, opts: &Opts) -> SimpleConnection {
    let s_connection = SettingConnection::new();
    s_connection.set_property_type(Some(&SETTING_WIRELESS_SETTING_NAME));
    s_connection.set_property_id(Some(&opts.ssid));
    s_connection.set_property_autoconnect(false);
    s_connection.set_property_interface_name(interface);

    let s_wireless = SettingWireless::new();
    s_wireless.set_property_ssid(Some(&(opts.ssid.as_bytes().into())));
    s_wireless.set_property_band(Some("bg"));
    s_wireless.set_property_hidden(false);
    s_wireless.set_property_mode(Some("ap"));

    let s_wireless_security = SettingWirelessSecurity::new();
    s_wireless_security.set_property_key_mgmt(Some("wpa-psk"));
    s_wireless_security.set_property_psk(Some(&opts.password));

    let address = IPAddress::new(libc::AF_INET, &opts.address, 24).unwrap();
    let s_ip4 = SettingIP4Config::new();
    s_ip4.add_address(&address);
    s_ip4.set_property_method(Some("manual"));

    let connection = SimpleConnection::new();

    connection.add_setting(&s_connection);
    connection.add_setting(&s_wireless);
    connection.add_setting(&s_wireless_security);
    connection.add_setting(&s_ip4);

    connection
}
