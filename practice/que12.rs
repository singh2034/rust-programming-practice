/*
Mini-Challenge C: The Range (for)
Task: Use a for loop and a range (1..11) to print the multiplication table for the number 5 (e.g., "5 x 1 = 5", "5 x 2 = 10"...).
Hint: 1..11 goes from 1 to 10.
*/

fn main () {
// set variable for the mentioned table number
    let table_of = 5;
    // start from 1, stops before 11.(n-1)
    for i in 1..11 {
        let result = table_of * i;
        println!("{table_of} x {i} = {result}");
    }
}
