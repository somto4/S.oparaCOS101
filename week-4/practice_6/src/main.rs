// Rust program to count numbers

use std::io;

fn main() {
    // Get the lower bound
    println!("Enter lower bound:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound: i32 = input1.trim().parse().expect("Please enter a valid number");

    // Get the upper bound
    println!("Enter upper bound:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound: i32 = input2.trim().parse().expect("Please enter a valid number");

    // Loop from lower_bound to upper_bound (upper_bound is exclusive)
    for x in lower_bound..upper_bound {
        println!("Count level is {}", x);
    }
}
