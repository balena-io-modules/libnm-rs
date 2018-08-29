extern crate glib;
extern crate nm;

use nm::*;

fn main() {
    //    let client = nm::Client::new(None).unwrap();

    let uuid = nm::utils_uuid_generate();

    println!("{}", uuid);

    println!("{}", nm::utils_is_uuid(&uuid));

    let type_ = nm::Setting::lookup_type(&nm::SETTING_DUMMY_SETTING_NAME);
    let obj = glib::Object::new(type_, &[]).unwrap();
    let dummy = obj.downcast::<nm::SettingDummy>().unwrap();

    let dummy2 = nm::SettingDummy::new();

    println!("{:?}", dummy);

    println!("{:?}", dummy2);
}
