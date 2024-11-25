use std::io;

fn main() {
    // Define the menu and prices
    let menu = [
        ("P", "Poundo Yam/Edinkaiko Soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    println!("Menu:");
    for (code, item, price) in menu.iter() {
        println!("{} = {} - ₦{}", code, item, price);
    }

    let mut total_price = 0;

    loop {
        println!("\nEnter the food code (or type 'done' to finish):");
        let mut food_code = String::new();
        io::stdin().read_line(&mut food_code).unwrap();
        let food_code = food_code.trim().to_uppercase();

        if food_code == "DONE" {
            break;
        }

        let mut found = false;
        for (code, item, price) in menu.iter() {
            if food_code == *code {
                found = true;

                println!("Enter the quantity for {}:", item);
                let mut quantity_input = String::new();
                io::stdin().read_line(&mut quantity_input).unwrap();
                let quantity: u32 = quantity_input.trim().parse().unwrap_or(0);

                total_price += price * quantity;
                println!("Added {} x {} (₦{})", quantity, item, price);
                break;
            }
        }

        if !found {
            println!("Invalid food code! Please try again.");
        }
    }

    // Check for discount
    if total_price > 10000 {
        let discount = total_price as f32 * 0.05;  // Calculate the discount as f32
        total_price = (total_price as f32 - discount) as i32; // Convert back to i32 after discount
        println!("A 5% discount has been applied!");
    }

    println!("\nTotal price: ₦{}", total_price);
}
