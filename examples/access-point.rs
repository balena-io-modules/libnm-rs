#![allow(unused_variables)]
#![allow(dead_code)]

extern crate nm;

use nm::*;

use anyhow::{anyhow, Result};
use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "wlan0")]
    interface: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let context = glib::MainContext::default();

    context.with_thread_default(|| -> Result<()> { context.block_on(run(&opts)) })?;

    Ok(())
}

async fn run(opts: &Opts) -> Result<()> {
    let client = Client::new_async_future().await?;
    let device = get_exact_device(&client, &opts.interface)?;
    if let Some(description) = device.get_description() {
        println!("Device found: {}", description);
    }
    Ok(())
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
