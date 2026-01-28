/*
Mini-Challenge B: The Countdown (while)
Task: Create a mutable variable count = 5. 
Use a while loop to print the number and subtract 1 each time. When it hits 0, print "Liftoff!".
*/

fn main () {
    let mut num:i8 = 5;
    while num !=0 {
        println!("{num}");
        num -= 1;
    }
    println!{"LIFT OFF!"};  
}