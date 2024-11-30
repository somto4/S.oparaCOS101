use std::io;

fn main() {
    loop {
        // Display the menu
        println!("Choose a calculation to perform:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        // Read user choice
        let choice = read_input().trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => calculate_trapezium(),
            2 => calculate_rhombus(),
            3 => calculate_parallelogram(),
            4 => calculate_cube(),
            5 => calculate_cylinder(),
            6 => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn calculate_trapezium() {
    println!("Enter the height:");
    let height: f64 = read_input().trim().parse().expect("Invalid number");
    println!("Enter the first base:");
    let base1: f64 = read_input().trim().parse().expect("Invalid number");
    println!("Enter the second base:");
    let base2: f64 = read_input().trim().parse().expect("Invalid number");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn calculate_rhombus() {
    println!("Enter the first diagonal:");
    let diagonal1: f64 = read_input().trim().parse().expect("Invalid number");
    println!("Enter the second diagonal:");
    let diagonal2: f64 = read_input().trim().parse().expect("Invalid number");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

fn calculate_parallelogram() {
    println!("Enter the base:");
    let base: f64 = read_input().trim().parse().expect("Invalid number");
    println!("Enter the altitude:");
    let altitude: f64 = read_input().trim().parse().expect("Invalid number");
    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn calculate_cube() {
    println!("Enter the length of a side:");
    let side: f64 = read_input().trim().parse().expect("Invalid number");
    let area = 6.0 * side.powi(2);
    println!("The surface area of the cube is: {:.2}", area);
}

fn calculate_cylinder() {
    const PI: f64 = std::f64::consts::PI;
    println!("Enter the radius:");
    let radius: f64 = read_input().trim().parse().expect("Invalid number");
    println!("Enter the height:");
    let height: f64 = read_input().trim().parse().expect("Invalid number");
    let volume = PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

