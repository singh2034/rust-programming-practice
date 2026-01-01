// fn main() {
//     let mut x = 5; //performed mutability to change the variable as per the need
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// shadowing

fn main() {
    let x = 5;
    let x = x + 1;

    // value calculated till here

    {
        // value used here
        let x = x * 2;
        println!("The value of x  in the inner scope is : {x}");
    }

    // first value is used here
    println!("The value of x is: {x}");
}
