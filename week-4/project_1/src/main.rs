
use std::io;

fn main() {
    println!("Enter the coefficients a, b, and c:");

    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    io::stdin().read_line(&mut a_input).expect("Failed to read input");
    io::stdin().read_line(&mut b_input).expect("Failed to read input");
    io::stdin().read_line(&mut c_input).expect("Failed to read input");

    let a: f64 = a_input.trim().parse().expect("Invalid input");
    let b: f64 = b_input.trim().parse().expect("Invalid input");
    let c: f64 = c_input.trim().parse().expect("Invalid input");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
}
