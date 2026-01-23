/*
Challenge 2: The Rectangle Area
Create a function called calculate_area.
=> It should take two parameters: length * Breadth (both u32).
=> It should return the area (u32).
=> In your main function, call this function and print the result.
*/

fn calculate_area(len:u32, bre:u32) -> u32 {
    len * bre
}

fn main () {
    let len = 5;
    let bre = 6;
    let area = calculate_area(len, bre);
    println!("The area of rectangle is: {area}");
}