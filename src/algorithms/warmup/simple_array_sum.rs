/*
Given an array of integers, can you find the sum of its elements?

Input Format

The first line contains an integer, , denoting the size of the array.
The second line contains space-separated integers representing the array's elements.

Output Format

Print the sum of the array's elements as a single integer.

Sample Input

6
1 2 3 4 10 11

Sample Output

31

Explanation

We print the sum of the array's elements, which is: 1 + 2 + 3 + 4 + 10 + 11 = 31.
*/

use std::io;

#[allow(unused)]
pub fn main() {
    let mut s_n = String::new();
    let mut s_array = String::new();

    io::stdin().read_line(&mut s_n).ok().expect("read error");
    io::stdin().read_line(&mut s_array).ok().expect("read error");

    let n: i32 = s_n.trim().parse().ok().expect("parse error");
    let it = s_array.trim().split( ' ');

    let mut sum: i64 = 0;
    for x in it {
        let c: i64 = x.parse().ok().expect("parse error");
        sum += c;
    }

    println!("{}", sum);
}