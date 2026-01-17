fn main() {
    println!("Hello, world!");
    another_function();
    parameter_function(5);
    print_label_measusrement(5, 'h');
}

fn another_function() {
    println!("Hello Another World!");
}

fn parameter_function (x:i8) {
println!("The value of x is : {x}");
}

fn print_label_measusrement (value:i8, unit_label: char) {
println!("The measurement is: {value}{unit_label}");
}