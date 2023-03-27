//use the io library from Rust's std library
use std::io;

fn main() {

    // print this message with the println macro, NOT function
    println!("Guess the number");

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

}