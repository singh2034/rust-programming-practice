// Let's code a fibonacci sequence.

fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}

// defince a signature function which takes a number n and returns a number
fn fibonacci(n: u32) -> u32 {
    // handle the zero case, if someone asks for 0th number, the answer is just 0.
    if n == 0 {
        return 0;
    }

    // initialise the variables, make 2 vars a&b make them mutable
    let mut a = 0;
    let mut b = 1;

    // Print the first two numbers to start the sequence
    print!("{}, {}", a, b);

    // The loop :- We need to calculate the sum and slide the number forward :- a+b = c; c+b =d. we start from 2 cause we already handled 0&1.
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;

        // Print the new number in the sequence
        print!(", {}", b);
        // note :- 2..=n :- from 2 upto and including n; _ is used because we do not need the loop counter variable
        /*
        Why print! instead of println!?

        => println! (with an n) adds a "newline" character, so every number would be on a new row.

        -> print! keeps everything on the same line, which looks more like a mathematical sequence.
        */
    }
    println!(); // Add a new line at the very end
    b // return the value, no semicolon means this is an expression and returns a value.
}
