extern crate clap;
extern crate futures;
extern crate gio;
extern crate glib;
extern crate libc;

extern crate nm;

use clap::{App, Arg};

use glib::translate::FromGlib;

use futures::prelude::*;

use nm::*;

const DEFAULT_SSID: &str = "AccessPoint";
const ADDRESS: &str = "192.168.42.1";

#[derive(Debug)]
struct Config {
    ssid: String,
    password: Option<String>,
    interface: Option<String>,
}

fn get_config() -> Config {
    let matches = App::new("access-point")
        .version("0.0.1")
        .arg(
            Arg::with_name("ssid")
                .short("s")
                .long("ssid")
                .value_name("ssid")
                .help(&format!("Access point SSID (default: {})", DEFAULT_SSID))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("password")
                .help("Access point password (default: none)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .value_name("interface")
                .help("WiFi interface name")
                .takes_value(true),
        )
        .get_matches();

    let ssid: String = matches
        .value_of("ssid")
        .map_or_else(|| DEFAULT_SSID.to_string(), String::from);

    let password: Option<String> = matches.value_of("password").map(String::from);

    let interface: Option<String> = matches.value_of("interface").map(String::from);

    Config {
        ssid,
        password,
        interface,
    }
}

fn main() {
    let config = get_config();

    let context = glib::MainContext::default();
    let loop_ = glib::MainLoop::new(Some(&context), false);

    context.push_thread_default();

    let client = Client::new(None).unwrap();

    let device = find_device(&client, &config.interface);

    let s_connection = SettingConnection::new();
    s_connection.set_property_type(Some(&SETTING_WIRELESS_SETTING_NAME as &str));
    s_connection.set_property_id(Some(&config.ssid as &str));
    s_connection.set_property_autoconnect(false);
    s_connection.set_property_interface_name(device.get_iface().as_ref().map(glib::GString::as_str));

    let s_wireless = SettingWireless::new();
    s_wireless.set_property_ssid(Some(&(config.ssid.as_bytes().into())));
    s_wireless.set_property_band(Some("bg"));
    s_wireless.set_property_hidden(false);
    s_wireless.set_property_mode(Some("ap"));

    let s_wireless_security = SettingWirelessSecurity::new();
    s_wireless_security.set_property_key_mgmt(Some("wpa-psk"));
    s_wireless_security.set_property_psk(Some(&config.password.unwrap() as &str));

    let s_ip4 = SettingIP4Config::new();
    let address = IPAddress::new(libc::AF_INET, ADDRESS, 24).unwrap();
    s_ip4.add_address(&address);
    s_ip4.set_property_method(Some("manual"));

    let connection = SimpleConnection::new();

    connection.add_setting(&s_connection);
    connection.add_setting(&s_wireless);
    connection.add_setting(&s_wireless_security);
    connection.add_setting(&s_ip4);

    if let Err(e) = connection.normalize() {
        panic!("Verification error: {:?}", e);
    }

    let l_clone = loop_.clone();

    let future = client.add_and_activate_connection_async_future(&connection, &device, None);
    let new_future = future
        .map(|(_con, active_con)| {
            active_con.connect_state_changed(move |active_con, state, _| {
                let state = ActiveConnectionState::from_glib(state as _);
                println!("Connection state: {:?}", state);

                match state {
                    ActiveConnectionState::Activated => {
                        println!("Connection successfully activated");
                        l_clone.quit();
                    }
                    ActiveConnectionState::Deactivated => {
                        println!("Connection NOT activated!");
                        let r_con = active_con.get_connection().unwrap();
                        r_con.delete(None).unwrap();
                        l_clone.quit();
                    }
                    _ => {}
                }
            });
        })
        .map_err(|(_con, e)| {
            eprintln!("{:?}", e);
        })
        .then(move |_| Ok(()));

    context.spawn_local(new_future);

    loop_.run();

    context.pop_thread_default();
}

fn find_device(client: &Client, interface: &Option<String>) -> Device {
    if let Some(ref interface) = *interface {
        match client.get_device_by_iface(interface) {
            Some(device) => device,
            _ => panic!("Interface not found: {}", interface),
        }
    } else {
        let devices = client.get_devices();

        let device = devices
            .iter()
            .find(|d| d.get_device_type() == DeviceType::Wifi);

        if let Some(device) = device {
            println!("WiFi device: {}", device.get_iface().unwrap());
            device.clone()
        } else {
            panic!("Could not find a WiFi device");
        }
    }
}
