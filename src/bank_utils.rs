use std::io::prelude::*;
use std::fs::File;


pub fn create_new_user(username: String) {
    let def_mon:u32 = 100;
    let file_name = format!("{username}.bank");
    let mut data_file = File::create(file_name).expect("creation failed");

    // Write contents to the file
    data_file
        .write(def_mon.to_string().as_bytes())
        .expect("write failed");
}
