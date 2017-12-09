/*
Input Format

Code that reads input from stdin is provided for you in the editor. There are lines of input, and each line contains a single integer.

Output Format

Code that prints the sum calculated and returned by solveMeFirst is provided for you in the editor.
*/

use std::io;

#[allow(unused)]
pub fn run() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    let mut _num_1: i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let mut _num_2: i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // print the sum
    // Hint: Type println!("{}", _num_1 + _num_2); below
    println!("{}", _num_1 + _num_2);
}