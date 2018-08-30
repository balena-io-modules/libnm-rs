extern crate futures_core;
extern crate glib;
extern crate nm;

use nm::*;

fn main() {
    //let client = nm::Client::new(None).unwrap();

    let uuid = nm::utils_uuid_generate();

    let type_ = &nm::SETTING_WIRED_SETTING_NAME;

    let type_g = nm::Setting::lookup_type(type_);
    let s_obj = glib::Object::new(type_g, &[]).unwrap();
    let s_base = s_obj.downcast::<nm::Setting>().unwrap();
    println!("{:?}", s_base);

    let con = nm::SimpleConnection::new();

    println!("{:?}", con);

    let s_con = nm::SettingConnection::new();

    s_con.set_property_id(Some("test1"));
    s_con.set_property_uuid(Some(&uuid));
    s_con.set_property_type(Some(type_));

    println!("{:?}", s_con);

    con.add_setting(&s_con);

    con.add_setting(&s_base);

    let as_dbus = con.to_dbus(nm::ConnectionSerializationFlags::ALL).unwrap();

    println!("[PRE ] {}", as_dbus);

    match con.normalize() {
        Ok(modified) => println!("Modified: {}", modified),
        Err(e) => panic!("Verification error: {:?}", e),
    }

    let as_dbus = con.to_dbus(nm::ConnectionSerializationFlags::ALL).unwrap();

    println!("[POST] {}", as_dbus);
}
