/** 
Challenge 2: Tuple Destructuring
Create a tuple named user_info that contains a string (a name), an integer (an age), and a boolean (representing if they are an admin).
=> Write a line of code to destructure this tuple into three separate variables: name, age, and is_admin.
=> Print a message using these variables (e.g., "User [name] is [age] years old").
**/

fn main () {
    // tuple created
    let user_info:(String, u8, bool) = (String::from("Alice"), 64, true);
    
    // destructure
    let (name, age, is_admin) = user_info;

    // print name
    println!("User {} is {} years old.\nAdmin Privilege = {}", name, age, is_admin);
}