use rand::Rng;
use std::{cmp, io};

pub fn test() {
    println!("Guess the number!");

    const MIN: u8 = 1;
    const MAX: u8 = 100;

    let secret_number: u8 = rand::thread_rng().gen_range(MIN..=MAX);

    loop {
        println!("Enter a valid number between {MIN} and {MAX} (both included):");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        if !(MIN..=MAX).contains(&guess) {
            println!("Invalid range, the number should be between {MIN} and {MAX} (both included)");
            continue;
        }

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
