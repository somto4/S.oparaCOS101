fn main() {
    // Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    // Age vector
    let age = vec![16,17,19,22,20,21,18,23];

    print!("\nAgd allocation:\n");

    //loop to ilterate elements in vector
    for i in 0..age.len()
    {
        // iterating through i on the vector
        print!("{} is {} year old\n",name[i],age[i]);
    }

}
