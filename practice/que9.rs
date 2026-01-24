/* 
Challenge 4: Multiple Returns (Tuples)
Write a function called get_stats that:

1. Takes two numbers (i32).

2. Returns a Tuple containing their sum and their product.

3. In main, destructure that tuple into two variables and print them.
*/

// get stats where returns a tuple containing their sum and their product.
fn get_stats (x: i32, y: i32) -> (i32, i32) {
     (x + y, x * y)
}

// fn main where it destructured into two var and prints.
fn main () {
    let (sum, product) = get_stats(5, 10);
    println!("The sum is : {sum} & The product is : {product}");
}