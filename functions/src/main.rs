fn main() {
    println!("Hello, world!");
    println!(" ");

    println!("Another Function example");
    another_function();
    println!(" ");

    println!("Parameter Function Example");
    parameter_function(5);
    println!(" ");

    println!("Parameter with predefined Example");
    print_label_measurement(5, 'h');
    println!(" ");

    println!("Statements & Expression Example");
    statements_and_expression();
    println!(" ");

    println!("Functions with return values example");
    call_five();
}

fn another_function() {
    println!("Hello Another World!");
}

fn parameter_function(x: i8) {
    println!("The value of x is : {x}");
}

fn print_label_measurement(value: i8, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expression() {
    // statements and expressions
    let y: u8 = {
        let x: u8 = 3;
        x + 1
    };
    println!("The value of y is : {y}");
}

// Functions with return values
fn five() -> i8 {
    5
}

fn call_five() {
    let x = five();
    println!("The value of x is : {x}");
}


