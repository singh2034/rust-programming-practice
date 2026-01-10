fn main () {
    // need to tell the rust compiler that, if the number is positive, if the answer is only positive then, u or it is neg or pos then i.
    let guess:u32 = "42".parse().expect("Not a number");
    println!("{guess}");
}