/* 
Challenge 4: Constant vs. Variable
Declare a constant for the speed of light ($299,792,458$ meters per second). 
Remember to use the correct naming convention and type annotation ($u32$).
Declare a mutable variable for current_speed starting at 0.Change current_speed to 1000.
*/

fn main () {
const SPEED_OF_LIGHT:u32 = 299_792_458;
let mut current_speed = 0;
println!("Starting Speed: {current_speed}");
current_speed = 1000;

println!("Speed: {current_speed} / Limit: {SPEED_OF_LIGHT}");
}
