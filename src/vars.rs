// Variables hold primitive data or refrences to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Rad";
    let mut age = 69;

    println!("[Vars.rs] My name is {} and I am {} years old", name, age);

    age = 70; 

    println!("[Vars.rs] After a year now I am {} years old", age);

    // Define constant

    const ID: i32 = 001;
    println!("[Vars.rs] ID: {}", ID);

    // Assign multiple vars

    let ( my_name, my_age ) = ("Rad", 69);
    println!("[Vars.rs] {} is {}", my_name, my_age);
}