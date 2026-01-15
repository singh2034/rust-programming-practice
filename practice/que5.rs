/*
Challenge 5: Numeric Operations
Write a small snippet that calculates the remainder of 43 divided by 5 and stores it in a variable. 
Then, create a floating-point variable x with the value 2.0 (using the default float type) and divide it by 3.0.
*/

fn main () {
    let remainder:u8 = 43 % 5;
    println!("{remainder}");
    let x = 2.0 / 3.0;
    println!("{x}");
}