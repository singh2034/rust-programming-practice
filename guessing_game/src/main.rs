use rand::Rng; //random number and range lib
use std::cmp::Ordering; // From standard lib - compare and ordering
use std::io; // From Standard library - Input/Output 

fn main() {
    println!("Welcome to the guess game!");
    println!("Guess the number!");
    println!(" ");

    println!("Hint :- The Numbers are between 1-50!")

    // This will generate random numbers in between range mentioned
    let secret_number = rand::thread_rng().gen_range(1..=50);
    // println!("The Secret Number is : {secret_number}");

    // loop will work as continue
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // u32 will convert the guess string to integer.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                println!(" ");
                continue;
            }
        };

        println!("you guessed: {guess}");

        println!(" ");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small Kid!"),
            Ordering::Greater => println!("Don't Get Cocky!"),
            Ordering::Equal => {
                println!("Not all treasures are Silver and Gold mate!");
                // this will quit the game after the correct guess
                break;
            }
        }
    }
}
