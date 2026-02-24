use std::io;

fn main() {
    io_name();
    ownernship_scope();
    string_push();
    string_ptr();
    assignment();
    clone_method();
    stack_only_data();
    ownership_and_functions();
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

// Variables and Data interacting with clone
fn clone_method() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("S1 = {s1}, s2 = {s2}");
}

// integers - stack only data

fn stack_only_data() {
    let x = 5;
    let y = x;
    println!("x = {x} & y = {y}");
    // Note :- In this data, all the other things contradicted because integers have only fixed amount of data, that's why it is okay to not use clone method or any other method we learned above.
}

// Ownership & Functions

fn ownership_and_functions() {
    let s = String::from("Hello"); // s comes into scope
    takes_ownership(s); //value of s moves into the functions
    // and s is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // i32 implement the copy trait
    // x does not move into the functions,
    // so it okay to use the x afterwards, as mentioned integers have know fixed value until mutated.

    fn takes_ownership(some_string: String) {
        //some_string comes into the scope
        println!("{some_string}");
    } // some_string goes out of the scope and `drop` is called. The backing memory is freed

    fn makes_copy(some_integer: i32) {
        //some_integer comes into scope
        println!("{some_integer}");
    } // Here, some_integer goes out of scope. Nothing special happens.
}
