mod bank_utils;
use std::io;

fn clean_name(name: String) -> String {
    let temp = name.trim().to_string().replace(" ", "_").replace("\n", "").to_lowercase();
    return temp;
}


fn register(){
    let mut username = String::new();

    println!("Please enter a username:");
    io::stdin().read_line(&mut username).expect("failed to get input");

    username = clean_name(username);

    bank_utils::create_new_user(username);

    println!("Please restart the program to login");

}

fn main() {

    let mut guess = String::new();

    println!("Would you like to:");
    println!("(R)egister or (L)ogin");

    io::stdin().read_line(&mut guess).expect("failed to get input");


    guess = guess.trim().to_string().to_lowercase();


   match &guess[..] {
       "r" => register(),
       _ => println!("Invalid input"),
   }
}
