#![allow(dead_code)]

use nm::*;

use anyhow::{anyhow, Result};
use futures_core::future::Future;

pub fn spawn_local<F: Future<Output = ()> + 'static>(f: F) {
    glib::MainContext::ref_thread_default().spawn_local(f);
}

pub fn get_device(client: &Client, interface: Option<&str>) -> Result<Device> {
    if let Some(interface) = interface {
        get_exact_device(client, interface)
    } else {
        find_wifi_device(client)
    }
}

pub fn print_device_info(device: &Device) {
    if let Some(interface) = device.iface() {
        println!(
            "{} [{}]",
            interface,
            &format!("{}", device.device_type()).to_lowercase()[12..],
        );
        if let Some(vendor) = device.vendor() {
            if let Some(product) = device.product() {
                if !vendor.is_empty() && !product.is_empty() {
                    println!("{} - {}", vendor, product);
                }
            }
        }
    }
}

fn get_exact_device(client: &Client, interface: &str) -> Result<Device> {
    if let Some(device) = client.device_by_iface(interface) {
        if device.device_type() != DeviceType::Wifi {
            Err(anyhow!("Not a Wi-Fi device: {}", interface))
        } else {
            Ok(device)
        }
    } else {
        Err(anyhow!("Interface not found: {}", interface))
    }
}

fn find_wifi_device(client: &Client) -> Result<Device> {
    let mut devices = client.devices();

    let position = devices
        .iter()
        .position(|d| d.device_type() == DeviceType::Wifi);

    if let Some(i) = position {
        Ok(devices.swap_remove(i))
    } else {
        Err(anyhow!("Cannot find a Wi-Fi device"))
    }
}
