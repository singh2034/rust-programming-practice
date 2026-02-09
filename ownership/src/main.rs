use std::io;

fn main() {
    io_name();
    ownernship_scope();
}

fn ownernship_scope() {
    // ownership :- only one owner of each value at a time, owner goes out of scope - value will be dropped
    // This is the scope of this value :- Scope Starts
    let a: &str = "Hello 1st Owner";
    {
        // This the scope of this value :- Scope Starts
        let b: &str = "Hello 1st Owner in a scope";
        println!("Owner in the scope: {b}");
        // Owner 
        println!("owner of 1st: {a}");
        // Scope Ends
    }
    println!("{a}");
    // Scope Ends
}

// simple input & output use of the std::io library
fn io_name() {
    println!("What is your name?");
    // Mutable heap style String used for Heap or dynamic memory.
    let mut name = String::new();
    // Taking the input from the user or in the terminal from the user
    io::stdin()
        .read_line(&mut name)
        .expect("Write a name, not a number");
    // cleaning up the result & printing the clean result without any new line or something
    let name = name.trim();
    println!("Hello, {name}!");
}
