fn main() {
    let number: i8 = 3;

    if number < 5 {
        println!("True!");
    } else {
        println!("False!!");
    }

    another();
}

fn another() {
    let number: i8 = 3;
    if number != 0 {
        println!("Number was something other than zero");
    }
}
