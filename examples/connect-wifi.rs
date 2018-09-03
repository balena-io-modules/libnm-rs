extern crate futures;
extern crate gio;
extern crate glib;

extern crate nm;

use glib::translate::FromGlib;

use futures::prelude::*;

use nm::*;

fn main() {
    let ssid = "My-Network-SSID";
    let password = "My-Network-Password";
    
    let context = glib::MainContext::default();
    let loop_ = glib::MainLoop::new(Some(&context), false);

    context.push_thread_default();

    let client = nm::Client::new(None).unwrap();

    let s_connection = nm::SettingConnection::new();
    s_connection.set_property_type(Some(&SETTING_WIRELESS_SETTING_NAME));
    s_connection.set_property_id(Some(ssid));

    let s_wireless = SettingWireless::new();
    s_wireless.set_property_ssid(Some(&(ssid.as_bytes().into())));

    let s_wireless_security = SettingWirelessSecurity::new();
    s_wireless_security.set_property_key_mgmt(Some("wpa-psk"));
    s_wireless_security.set_property_psk(Some(password));

    let connection = nm::SimpleConnection::new();

    connection.add_setting(&s_connection);
    connection.add_setting(&s_wireless);
    connection.add_setting(&s_wireless_security);

    if let Err(e) = connection.normalize() {
        panic!("Verification error: {:?}", e);
    }

    let device = client.get_device_by_iface("wlo1").unwrap();

    let l_clone = loop_.clone();

    let future = client.add_and_activate_connection_async_future(&connection, &device, None);
    let new_future = future
        .map(|(_con, active_con)| {
            active_con.connect_state_changed(move |_, state, _|{
                let state = ActiveConnectionState::from_glib(state as _);
                match state {
                    ActiveConnectionState::Activated => {
                        println!("Connection successfully activated.");
                        l_clone.quit();
                    },
                    ActiveConnectionState::Deactivated => {
                        println!("Connection NOT activated!");
                        l_clone.quit();
                    },
                    _ => {}
                }
            });
        })
        .map_err(|(_con, e)| {
            eprintln!("{:?}", e);
        })
        .then(move |_| {
            Ok(())
        });

    context.spawn_local(new_future);

    loop_.run();

    context.pop_thread_default();
}
