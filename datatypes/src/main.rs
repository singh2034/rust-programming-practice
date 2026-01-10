fn main () {
    // need to tell the rust compiler that, if the number is positive, if the answer is only positive then, u or it is neg or pos then i.
    
    // u = unsigned = only positive
    let guess_unsigned:u32 = "42".parse().expect("Not a number");
    // i = signed = postive or negative
    let guess_signed:i32 = "-42".parse().expect("Not a number");
    println!("Unsigned Number or Only Positive Number = {guess_unsigned} \nSigned Number = {guess_signed}");
}