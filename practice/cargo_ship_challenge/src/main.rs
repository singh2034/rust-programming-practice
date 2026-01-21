/*
The "Cargo Ship" Challenge 🚢
You are writing a small program to manage a cargo ship's inventory.
The Constant: Define a constant called MAX_WEIGHT with a value of 10,000 (use $u32$).

The Array: Create an array called items_weight containing the weights of 3 items: 3500, 4200, and 1800.

The Calculation: Calculate the total weight of these 3 items by adding them together and store it in a variable called current_total.

The Tuple: Create a tuple called ship_status. It should hold:The current_total (as a number).

A boolean that is true if current_total is less than or equal to MAX_WEIGHT, and false otherwise.

The Shadowing: Use shadowing to convert the current_total (which is a number) into a String that says "Full" or just use a new string variable to print the status.

The Print: Print a message like: "Ship weight: [total]. Safe to sail: [boolean]."
*/

fn main() {

    // Maximum Weight of Cargo
    const MAX_WEIGHT:u32 = 10_000;
    
    // Array containing
    let items_weight:[u32; 3] = [3500, 4200, 1800];
    
    // current total of all the weights
    let current_total:u32 = items_weight[0] + items_weight[1] + items_weight[2];

    // boolean
    let is_safe = current_total <= MAX_WEIGHT;
    
    // tuple
    let ship_status = (current_total, is_safe);

    // Destructing & print
    let (weight, safe) = ship_status;
    println!("Ship Weight: {weight}");
    println!("Safe to Sail: {safe}");   
}
