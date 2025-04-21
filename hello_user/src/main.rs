/*
 2) Write a program that reads a person's name and displays a welcome message for them:  
Example:  
What is your name? João da Silva  
Hello João da Silva, it's a pleasure to meet you!
*/

use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name: ");

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

    println!("Hello {} nice to meet you !", name.trim());
}
