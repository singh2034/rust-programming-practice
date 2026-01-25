// main function with other ones
fn main() {
    let number: i8 = 3;

    if number < 5 {
        println!("True!");
    } else {
        println!("False!!");
    }

    another();
    else_if();
    loops_return();
}

fn another() {
    let number: i8 = 3;
    if number != 0 {
        println!("Number was something other than zero");
    }
}

// if & else if & else keys/functions

fn else_if() {
    let number: i8 = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4,3 & 2.");
    }
}

// returning values with loops || loops || break

fn loops_return() {
    let mut counter: i8 = 0;
    let result: i8 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is : {result}");
}
