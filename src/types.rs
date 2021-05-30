/* 
    Primitive Types:
- Integers: u8, i8, u16, i16, i32, u64, i64, u128, i128 (number of bits they take in memory)
- Floats: f32, f64
- Boolean: bool
- Character: char
- Tuples
- Arrays

    Rust is a statically typed language, which means that it must know the types of all variables at compile time,
     however,
     the compiler can usually infer what type we want to use based on the value and how we use it.

*/

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit types
    let z: i64 = 999999999999;

    // Max Size:
    println!("[Types.rs] Max i64: {0} example: {1}", std::i64::MAX, z);
    println!("[Types.rs] Max i32: {0} example: {1}", std::i32::MAX, x);

    // Boolean:
    let is_active = true;

    // Get boolean from expression:
    let is_higher = 69.0 > 69.1;

    // Chars

    let smile = '\u{1F603}';

    println!("[Types.rs] {:?}", (x, y, z, is_active, is_higher, smile));

}