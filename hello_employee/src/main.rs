/*
 3) Create a program that reads an employee's name and salary, and displays a message at the end.  
Example:  
Employee's Name: Maria do Carmo  
Salary: 1850.45  
The employee Maria do Carmo has a salary of R$1850.45 in June.
*/

use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name: ");

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

    let mut salary = String::new();

    println!("Enter your salary: ");

    io::stdin()
    .read_line(&mut salary)
    .expect("Failed to read line");

    let mut month = String::new();

    println!("Enter the last month you worked: ");

    io::stdin()
    .read_line(&mut month)
    .expect("Failed to read line");

    println!("The employee {} has a salary of {} in {}", name, salary, month);
}
