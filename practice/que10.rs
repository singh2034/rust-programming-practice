/*
Task: Write a program where you have a variable marks = 85. 
Use an if/else expression to assign a string "Pass" or "Fail" to a variable named status.
*/

fn main () {
    // variable
    let marks = 85;
    // condition and inside {} do not use ;, because they are expressions and are expected to return something when call. NOT A STATEMENT.
    let result = if marks >= 55 {"Pass"} else {"Fail"};

    println!("{result}");
}