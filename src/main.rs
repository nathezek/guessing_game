// This is my first rust game 😁 //

// extenal dependencies
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

// ------------------------- A function that prompts the user to continue to the next level or challenge ------------------------ //
fn prompt_to_continue() -> bool {
    loop {
        print!("Continue to the next level? [y/N]:");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read_line");

        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "y" | "Y" | "yes" => return true,
            "n" | "N" | "no" => return false,
            _ => println!("Invalid input, please enter 'y' or 'n'."),
        }
    }
}

fn play( level_num: u32, max_range: u32) -> bool {

    println!("\n--- Level {} (Guess a number between 1 and {}) ---", level_num, max_range);
        let secret_number = rand::thread_rng().gen_range(1, max_range + 1);

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to real line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🥳");
                return prompt_to_continue();
                
            }
        }
    }
}



fn main () {
    let game_title = r#"
              ██████╗ ██╗   ██╗███████╗███████╗███████╗██╗███╗   ██╗ ██████╗      ██████╗  █████╗ ███╗   ███╗███████╗
             ██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝██║████╗  ██║██╔════╝     ██╔════╝ ██╔══██╗████╗ ████║██╔════╝
             ██║  ███╗██║   ██║█████╗  ███████╗███████╗██║██╔██╗ ██║██║  ███╗    ██║  ███╗███████║██╔████╔██║█████╗  
             ██║   ██║██║   ██║██╔══╝  ╚════██║╚════██║██║██║╚██╗██║██║   ██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  
             ╚██████╔╝╚██████╔╝███████╗███████║███████║██║██║ ╚████║╚██████╔╝    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗
              ╚═════╝  ╚═════╝ ╚══════╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
                                                                                                        
    "#;

    println!("{}", game_title);
    println!("Guess the number!");

    if play(1, 10) {
        println!("Moving to Level 2...");
    }

    play(1, 50);
    println!("Thanks for playing");
}


