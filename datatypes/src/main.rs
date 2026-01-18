fn main() {
    // need to tell the rust compiler that, if the number is positive, if the answer is only positive then, u or it is neg or pos then i.

    // u = unsigned = only positive
    let guess_unsigned: u32 = "42".parse().expect("Not a number");
    // i = signed = postive or negative
    let guess_signed: i32 = "-42".parse().expect("Not a number");
    println!(
        "Unsigned Number or Only Positive Number = {guess_unsigned} \nSigned Number = {guess_signed}"
    );

    // floating
    let x = 2.4;
    let y: f32 = 3.4;

    println!("The value of x is : {x} & y is : {y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
}
