extern crate clap;
extern crate futures;
extern crate gio;
extern crate glib;

extern crate nm;

use clap::{App, Arg};

use glib::translate::FromGlib;

use futures::prelude::*;

use nm::*;

const DEFAULT_SSID: &str = "AccessPoint";

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
        ).arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("password")
                .help("Access point password (default: none)")
                .takes_value(true),
        ).arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .value_name("interface")
                .help("WiFi interface name")
                .takes_value(true),
        ).get_matches();

    let ssid: String = matches
        .value_of("portal-ssid")
        .map_or_else(|| DEFAULT_SSID.to_string(), String::from);

    let password: Option<String> = matches.value_of("password").map(String::from);

    let interface: Option<String> = matches.value_of("interface").map(str::to_string);

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

    loop_.run();

    context.pop_thread_default();
}
