// main function with other ones
fn main() {
    let number: i8 = 3;

    if number < 5 {
        println!("True!");
    } else {
        println!("False!!");
    }

    println!("Number Function!!");
    another();
    println!(" ");

    println!("Conditions with else and if");
    else_if();
    println!(" ");

    println!("Return values with loops");
    loops_return();
    println!(" ");

    println!("Loop Within Loop");
    loop_within_loop();
    println!(" ");

    println!("while loops");
    while_loop();
    println!(" ");
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

// Loop Labels || Loop withing the loop

fn loop_within_loop() {
    let mut count: i8 = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining: i8 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");
}

// while loops

fn while_loop() {
    let mut number: i8 = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!");
}
