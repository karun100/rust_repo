
use std::io;
use rand::Rng;
extern crate rand;

fn main() {
// take input from the user
let random_number = rand::thread_rng().gen_range(1,200);
loop {
println!("please enter the number");
let mut guess_number = String::new();
io::stdin().read_line(&mut guess_number);
println!("input number is : {}", guess_number);
let guess_number:u32 = match guess_number.trim().parse() {
    Ok(num)=>num,
    Err(_) =>continue,
};

// convert string to int
// generate random number
// Loop for keep guessing multiple times
// matching guess with the random number
//exit from loop if guessed number is correct
    println!("Hello, world!");
}
}
