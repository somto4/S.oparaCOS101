use std::io;

fn main() {
    println!("Enter experience level (experienced or inexperienced):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience = experience_input.trim();

    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Invalid input");

    let incentive = if experience == "experienced" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12  // Monthly incentive converted to annual
        } else {
            0 // Handle any additional conditions if needed
        }
    } else {
        100_000
    };

    println!("The annual incentive is: ₦{}", incentive);
}