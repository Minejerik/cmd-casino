mod bank_utils;
use rand::Rng;
use std::io;

static mut MONEY: u32 = 100;

static mut CAN_PLAY: bool = false;

static mut USERNAME: String = String::new();

fn clean_name(name: String) -> String {
    let temp = name
        .trim()
        .to_string()
        .replace(" ", "_")
        .replace("\n", "")
        .to_lowercase();
    return temp;
}

fn login() {
    let mut username = String::new();
    unsafe { CAN_PLAY = true };

    println!("Please enter a username:");
    io::stdin()
        .read_line(&mut username)
        .expect("failed to get input");

    username = clean_name(username);

    unsafe { USERNAME = username.clone() };

    unsafe { MONEY = bank_utils::get_balance(username) };
}

fn register() {
    let mut username = String::new();
    unsafe { CAN_PLAY = true };

    println!("Please enter a username:");
    io::stdin()
        .read_line(&mut username)
        .expect("failed to get input");

    username = clean_name(username);

    bank_utils::create_new_user(username);

    println!("Please restart the program to login");
}

fn play() {
    loop {
        let mut heads: bool = false;
        let mut tails: bool = false;

        println!("You have {} dollars", unsafe { MONEY });
        println!("How much would you like to bet?");
        let mut bet = String::new();
        io::stdin()
            .read_line(&mut bet)
            .expect("failed to get input");

        let bet: u32 = bet.trim().parse().expect("failed to parse");

        if bet > unsafe { MONEY } {
            println!("You don't have enough money to bet that much");
            break;
        }

        let mut side_to_bet = String::new();
        println!("Would you like to bet on (H)eads or (T)ails or (E)xit? ");
        io::stdin()
            .read_line(&mut side_to_bet)
            .expect("failed to get input");

        side_to_bet = side_to_bet.trim().to_string().to_lowercase();

        match &side_to_bet[..] {
            "h" => heads = true,
            "t" => tails = true,
            "e" => break,
            _ => println!("Invalid input"),
        }

        if heads == false && tails == false {
            break;;
        }

        let mut rng = rand::thread_rng();
        let flip = rng.gen_range(0..2);

        if flip == 0 && heads == true {
            println!("You win!");
            unsafe { MONEY += bet };
        } else if flip == 1 && tails == true {
            println!("You win!");
            unsafe { MONEY += bet };
        } else {
            println!("You lose!");
            unsafe { MONEY -= bet };
        }
    }

}

fn main() {
    let mut guess = String::new();

    println!("Would you like to:");
    println!("(R)egister or (L)ogin");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to get input");

    guess = guess.trim().to_string().to_lowercase();

    match &guess[..] {
        "r" => register(),
        "l" => login(),
        _ => println!("Invalid input"),
    }

    if !unsafe { CAN_PLAY } {
        return;
    } else {
        play();
    }
}
