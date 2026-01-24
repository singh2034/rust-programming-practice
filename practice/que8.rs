/*
Challenge 3: Shadowing with Functions
Write a program where:
1. In main, you have a variable price = 100.
2. You have a function called apply_discount that takes an integer and returns that integer minus 10.
3. In main, shadow the price variable by calling the apply_discount function.
4. Print the final price.
*/

fn main () {
    let price:u32 = 100;
    println!("Before Discounted Price: {price}");
    
    {
        let price = apply_discount(price);
        println!("After Discounted Price: {price}");
    }
    
}

// returning value doesnot have any semicolon, cause it is an expression not an statement. It's use is to return a value or the calculation.
fn apply_discount(price: u32) -> u32 {
    price - 10
}