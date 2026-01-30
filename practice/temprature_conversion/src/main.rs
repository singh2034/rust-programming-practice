// Let's make a Temprature conversion from Fahrenheit to Celsius and Celsius to Fahrenheit.
use std::io;

fn main() {
    println!("Type 'C' for Celsius and 'F' for Fahrenheit: ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim(); //remove the extra enter line \n

    println!("Enter the value.");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    // convert string input to a floating number (f64)
    let value: f64 = value.trim().parse().expect("Please type a number!");

    if choice == "C" {
        let result = (value * 1.8) + 32.0;
        println!("{:.2}^0 C is {:.2}^0 F", value, result);
    } else if choice == "F" {
        let result = (value - 32.0) * 5.0 / 9.0;
        println!("{:.2}^0 F is {:.2}^0 C", value, result);
    } else {
        println!("Wrong input!");
    }
}
