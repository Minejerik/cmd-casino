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


pub fn get_balance(username: String) -> u32 {
    let file_name = format!("{username}.bank");
    let mut data_file = File::open(file_name).expect("open failed");

    let mut contents = String::new();
    data_file
        .read_to_string(&mut contents)
        .expect("read failed");

    let balance:u32 = contents.parse().unwrap();

    return balance;
}

pub fn save_balance(username: String, balance: u32) {
    let file_name = format!("{username}.bank");
    let mut data_file = File::create(file_name).expect("creation failed");

    // Write contents to the file
    data_file
        .write(balance.to_string().as_bytes())
        .expect("write failed");
}