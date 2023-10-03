mod bank_utils;
use std::io;

fn main() {

    let mut guess = String::new();

    println!("Would you like to:");
    println!("(R)egister or (L)ogin");

    io::stdin().read_line(&mut guess).expect("failed to get input");


    println!("{}", guess);

    guess = guess.trim().to_string().to_lowercase();


   match &guess[..] {
       "r" => println!("temp"),
       _ => println!("Invalid input"),
   }
}
