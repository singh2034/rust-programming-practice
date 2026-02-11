use std::io;

fn main() {
    io_name();
    ownernship_scope();
    string_push();
    string_ptr();
    assignment();
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
    let mut name: String = String::new();
    // Taking the input from the user or in the terminal from the user
    io::stdin()
        .read_line(&mut name)
        .expect("Write a name, not a number");
    // cleaning up the result & printing the clean result without any new line or something
    let name: &str = name.trim();
    println!("Hello, {name}!");
}

// push_str function in memory || Dynamic memory allocation || HEAP || String::from() method is used for this.

fn string_push() {
    let mut s: String = String::from("Hello");
    s.push_str(", World!"); //push appends the new word in it.
    println!("{s}");
}

fn string_ptr() {
    let s1: String = String::from("Hello");
    let s2: String = s1;
    println!("{s2}");
    // println!("{s1}"); //dropped memory so it will be the compile time error.
}

// scope and assignment
fn assignment() {
    let mut s = String::from("Hello"); //this will show the warning as the value of S is shifted in the 2nd line. But it will still work
    s = String::from("Ahoy!");
    println!("{s}, world!");
}
