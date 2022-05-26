use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This code mostly followed Chapter 2 of the Rust book with some minor additions :)");
    guessing_game();
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_counter = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_counter = guess_counter + 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_counter = guess_counter + 1;
            }
            Ordering::Equal => {
                guess_counter = guess_counter + 1;
                println!("You win!\nYou guessed: {} time(s)", guess_counter);
                break;
            }
        }
    }
}
