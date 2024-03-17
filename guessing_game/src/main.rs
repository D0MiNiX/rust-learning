use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("secret number: {secret_number}");

    // println!("\nEnter number:");
    // let mut guess: String = String::new();

    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    // let guess: u32 = guess.trim().parse().expect("Please input number.");

    // println!("You guessed: {guess}");

    loop {
        println!("\nEnter number:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please input number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please enter a valid number!! ERR: {}", err.to_string());
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // Another way to print and trim new line
    // println!("You guessed: {}", guess.trim());
}
