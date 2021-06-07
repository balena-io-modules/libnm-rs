extern crate nm;

mod common;

use nm::*;

use anyhow::{anyhow, Context, Result};
use clap::Clap;
use futures_channel::oneshot;
use std::cell::RefCell;
use std::rc::Rc;

use glib::translate::FromGlib;

use common::*;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    interface: Option<String>,

    #[clap(short, long, default_value = "NMAccessPoint")]
    ssid: String,

    #[clap(short, long)]
    password: Option<String>,

    #[clap(short, long)]
    address: Option<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let context = glib::MainContext::default();

    context.block_on(run(opts))
}

async fn run(opts: Opts) -> Result<()> {
    let client = Client::new_async_future()
        .await
        .context("Failed to create NM Client")?;

    let device = get_device(&client, opts.interface.as_deref())?;

    print_device_info(&device);

    let connection = create_connection(device.iface().as_deref(), &opts)?;

    let active_connection = client
        .add_and_activate_connection_async_future(Some(&connection), &device, None)
        .await
        .context("Failed to add and activate connection")?;

    let (sender, receiver) = oneshot::channel::<Result<()>>();
    let sender = Rc::new(RefCell::new(Some(sender)));

    active_connection.connect_state_changed(move |active_connection, state, _| {
        let sender = sender.clone();
        let active_connection = active_connection.clone();
        spawn_local(async move {
            let state = unsafe { ActiveConnectionState::from_glib(state as _) };
            println!("Active connection state: {:?}", state);

            let exit = match state {
                ActiveConnectionState::Activated => {
                    println!("Successfully activated");
                    Some(Ok(()))
                }
                ActiveConnectionState::Deactivated => {
                    println!("Connection deactivated");
                    if let Some(remote_connection) = active_connection.connection() {
                        Some(
                            remote_connection
                                .delete_async_future()
                                .await
                                .context("Failed to delete connection"),
                        )
                    } else {
                        Some(Err(anyhow!(
                            "Failed to get remote connection from active connection"
                        )))
                    }
                }
                _ => None,
            };
            if let Some(result) = exit {
                let sender = sender.borrow_mut().take();
                if let Some(sender) = sender {
                    sender.send(result).expect("Sender dropped");
                }
            }
        });
    });

    receiver.await?
}

fn create_connection(interface: Option<&str>, opts: &Opts) -> Result<SimpleConnection> {
    let connection = SimpleConnection::new();

    let s_connection = SettingConnection::new();
    s_connection.set_type(Some(&SETTING_WIRELESS_SETTING_NAME));
    s_connection.set_id(Some(&opts.ssid));
    s_connection.set_autoconnect(false);
    s_connection.set_interface_name(interface);
    connection.add_setting(&s_connection);

    let s_wireless = SettingWireless::new();
    s_wireless.set_ssid(Some(&(opts.ssid.as_bytes().into())));
    s_wireless.set_band(Some("bg"));
    s_wireless.set_hidden(false);
    s_wireless.set_mode(Some(&NM_SETTING_WIRELESS_MODE_AP));
    connection.add_setting(&s_wireless);

    if let Some(password) = &opts.password {
        let s_wireless_security = SettingWirelessSecurity::new();
        s_wireless_security.set_key_mgmt(Some("wpa-psk"));
        s_wireless_security.set_psk(Some(password));
        connection.add_setting(&s_wireless_security);
    }

    let s_ip4 = SettingIP4Config::new();
    if let Some(address) = &opts.address {
        let address =
            IPAddress::new(libc::AF_INET, address, 24).context("Failed to parse address")?;
        s_ip4.add_address(&address);
        s_ip4.set_method(Some(&SETTING_IP4_CONFIG_METHOD_MANUAL));
    } else {
        s_ip4.set_method(Some(&SETTING_IP4_CONFIG_METHOD_SHARED));
    }
    connection.add_setting(&s_ip4);

    Ok(connection)
}
