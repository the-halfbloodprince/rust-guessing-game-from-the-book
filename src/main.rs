// tells Rust rand is an external dependency, and also does 'use rand'
extern crate rand;

//use the io library from Rust's std library
use std::io;
// use the Rng trait from the rand library
use rand::Rng;

fn main() {

    // print this message with the println macro, NOT function
    println!("Guess the number");

    // generate the random number
    let secret_num = rand
                        // get a random number generator local to the current thread of execution and seeded by the OS
                        ::thread_rng()
                        // call it's gen_range function which returns a number
                        .gen_range(1..101);

    // declare a mutable string 'guess' with the new function in the String type
    let mut guess = String::new();

    // call the stdin() in io library which gives us an object of Stdin
    io::stdin()
        // read line with the Stdin object into the mutable guess variable
        .read_line(&mut guess)
        // crash the program and show the below message, if there was any error
        .expect("Invalid input");

    // template formatting into string with placeholders
    println!("You guessed: {}", guess);
    println!("The secret number was: {}", secret_num);

}