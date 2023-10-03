mod bank_utils;
use std::io;

fn main() {

    let mut guess = String::new();

    println!("Would you like to:");
    println!("(R)egister or (L)ogin");

    io::stdin().read_line(&mut guess).expect("failed to get input");

    guess = guess.to_lowercase();


    if guess == "r".to_string(){
        println!("register");
        
    } else if guess == "l" {
        println!("login");
        
    } else {
        println!("Invalid input!");
        return
    }
}
