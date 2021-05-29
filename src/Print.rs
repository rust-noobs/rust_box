pub fn run() {
    // Printing
    println!("[Print.rs] Hello!");
    
    // Basic Formatting

    println!("[Print.rs] {} and {} are {} friends", "Cannon", "Rad", 2);

    // Positional Arguments

    println!("[Print.rs] {0} is a friend of {1}. {0} likes to code!", "Rad", "Cannon");

    // Named Arguments
    
    println!("[Print.rs] {name} likes to learn {language}", name = "Rad", language = "Rust");

    // Placeholder traits

    println!("[Print.rs] Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait

    println!("[Print.rs] {:?}", (69, true, "yo"));

    // Basic Math

    println!("[Print.rs] 20 / 5 = {}", 20 / 5);
}