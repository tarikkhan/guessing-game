use colored::*;
use rand::Rng;
use std::cmp::Ordering;

pub fn game(difficulty: i32) {
    let mut counter = difficulty;
    //println!("{}", "Guess the number:".green());
    println!("Number of guesses you get: {counter}");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    println!("Type in your guess:");
    loop {
        let mut guess = String::new();
        if counter <= 3 {
            tips(secret_number, counter);
        }

        counter -= 1;
        std::io::stdin()
            .read_line(&mut guess)
            .expect("could not read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {} Please type in a valid number", e);
                println!("Guesses left: {counter}");
                continue;
            }
        };

        if guess > 100 || guess < 1 {
            println!("The secret number is between 1 and 100");
            println!("Guesses left: {counter}");
            continue;
        }
        if counter <= 0 {
            println!("You have no more guesses left;");
            println!("{}", "GAME OVER!!!".red().bold());
            println!("Secret number was: {secret_number}");
            break;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess is smaller than the secret number. Guess again!");
                println!("Guesses left: {counter}");
                continue;
            }
            Ordering::Greater => {
                println!("Your guess is greater than the secret number. Guess again!");
                println!("Guesses left: {counter}");
                continue;
            }
            Ordering::Equal => {
                println!("Your guess is Correct. {}", "YOU WIN!!!".green().bold());
                break;
            }
        }
    }
}

fn tips(secret_number: i32, counter: i32) {
    if counter == 3 {
        if (secret_number / 5) == 0 {
            println!("The secret number is divisible by 5");
        } else if (secret_number / 3) == 0 {
            println!("The secret number is divisible by 3");
        } else {
            println!("The secret number is not divisible by 5 or 3");
        }
    }

    if counter == 1 {
        if (secret_number / 2) == 0 {
            println!("The secret number is even");
        } else {
            println!("The secret number is odd");
        }
    }
}
