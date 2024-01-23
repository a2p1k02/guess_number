use std::io::{stdin, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();

    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();

    println!("Welcome to Guessing Game {}!", name.trim());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;

    let mut answer = String::new();
    print!("Do you want to play?(y/n) ");
    std::io::stdout().flush().unwrap();

    stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_lowercase() == "y" {
        loop {
            println!("Try to guess number: ");

            let mut guess_number = String::new();
            stdin().read_line(&mut guess_number).unwrap();
            let guess_number: i32 = guess_number.trim().parse().unwrap();

            match guess_number.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                    counter += 1;
                }
                Ordering::Greater => {
                    println!("Too greater!");
                    counter += 1;
                }
                Ordering::Equal => {
                    println!("Congratulations! You are win {}!\nYou guessed for {} tries", name.trim(), counter);
                    break;
                }
            }
        }
    } else {
        println!("Okay, goodbye {}!", name.trim());
    }
}
