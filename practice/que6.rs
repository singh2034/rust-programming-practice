/* Challenge 1: The Temperature Converter
Write a function called celsius_to_fahrenheit that : Takes one parameter: temp (a float, f64).
Returns the converted temperature (also an f64).Formula: (Celsius * 1.8) + 32.
Requirements : Use an expression for the return value (don't use the return keyword). 
Add a doc comment (///) explaining what the function does. */

// Function Formula
fn celsius_to_fahrenheit(temp: f64) -> f64 {
(temp * 1.8) + 32.0
}

// Static Input
fn main () {
    let boiling_c = 100.0;
    let boiling_f = celsius_to_fahrenheit(boiling_c);
    println!("{}^C is {}^F", boiling_c, boiling_f);
}