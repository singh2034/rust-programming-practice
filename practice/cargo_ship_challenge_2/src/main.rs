/*
The Challenge: Create a program that:
Has an array of 4 cargo weights: [10, 20, 30, 40].
Uses a for loop to look at each weight.
If a weight is exactly 30, print "Found the special crate! Skipping...".
For all other weights, print "Loading crate of weight: {weight}".
*/

fn main () {
    // Array Containing weights
    let items_weight:[i16; 4] = [10, 20, 30, 40];

    // For loop to look at each weight
    for weight in items_weight {
        if weight == 30 {
            println!("Found the special crate! Skipping...");
        } else {
            println!("Loading crate of weight: {weight}");
        }
    }
}

// Note :- My mistake, I was using items_weight which is an array instead of i which is used for looking through each number.