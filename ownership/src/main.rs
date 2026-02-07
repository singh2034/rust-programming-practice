use std::io;

fn main() {
    io_name();
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
