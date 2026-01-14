/**
Challenge 3: Array Indexing & Safety

Create an array of the first five prime numbers.
Write a line of code that accesses the third element and prints it.
What happens if you try to access the element at index 10? 
(You don't have to write the code for the error, just explain what Rust does to protect you).

**/

fn main () {
    let prime = [2,3,5,7,11];
    let result = prime[2];
    println!("Third Element is = {}", result);
    // If I try to access the index 10, it will show an error saying about array indexing limit.
}