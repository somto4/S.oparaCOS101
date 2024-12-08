fn main() {
    
    // mutable array
    let mut colors = ["red", "green", "yellow" , "white"];

    println!("\n0riginal array = {:?}" colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
