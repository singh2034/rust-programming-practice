/**
Challenge 1: Variable Shadowing & Types 
Create a variable named data and assign it the string value "100". 
Then, use shadowing to transform that same variable name into an integer type ($i32$) with the value 100. 
Finally, add 50 to it and print the result.
**/

fn main () {
    println!("Hello Rustceans!");
    let data = "100";
    println!("Old Data = {data}");
    println!(" ");

    // Shadowing within the braces, results to a separate answer
    {
        println!("Shadow result: within the braces function ");
        let data:i32 = data.parse().expect("Not a Number");
        let result = data + 50;
        println!("New Data = {result}");
        println!(" ");
    }

    // shadowing globally
    println!("Shadow result: within the global function");
    let data:i32 = data.parse().expect("Not a valid number!");
    let result = data + 50;
    println!("Final New Data = {result}")
}