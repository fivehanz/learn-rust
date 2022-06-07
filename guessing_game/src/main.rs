use rand::Rng;
use std::{cmp::Ordering, io::Error, io, io::ErrorKind};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("The secret number will be between 1 and 100, got {}.", value)
                )
            );
        }

        // return result 
        Ok(Guess { value })
    }

    // use getter so that explicit assingment of private value is not possible
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(value) => value,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            },
            Err(_) => {
                println!("Please input a Number.");
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("guess is smaller than secret number"),
            Ordering::Greater => println!("guess is greater than secret number"),
            Ordering::Equal => {
                println!("You win, your guess is correct!");
                break;
            }
        }
    }
}
