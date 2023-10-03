mod bank_utils;
use std::io;

// pub use crate::bank_utils;


fn main() {

    let mut guess = String::new();

    println!("Would you like to:");
    println!("(R)egister or (L)ogin");

    io::stdin().read_line(&mut guess).expect("failed to get input");


    bank_utils::create_new_user("temp".to_string());
}
