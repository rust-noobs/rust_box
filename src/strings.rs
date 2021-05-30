// Primitive str: Immutable fixed length string somewhere in memory
// String: Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut name = String::from("Rad ");

    // Get length
    println!("[Strings.rs] Name Length: {}", name.len());

    // Push char
    name.push('i');

    // Push string
    name.push_str("s Epic");

    // Capacity in bytes
    let capacity = name.capacity();

    println!("[Strings.rs] {0}: Contains {1} bytes", name, capacity);

    // Check if empty

    let is_empty = name.is_empty();

    println!("[Strings.rs] {0}: Is empty: {1}", name, is_empty);

    // If contains a certain word:

    let contains_rad = name.contains("Rad");

    println!("[Strings.rs] {0}: Contains word Rad is {1}", name, contains_rad);

    // Replace:

    let new_name = name.replace("epic", "a legend");

    println!("[Strings.rs] New Name: {}", new_name);

    // Loop through string:

    let mut i: i8 = 0;

    for word in name.split_whitespace() {
        i += 1;
        println!("[Strings.rs] Word {index} is \"{w}\"", index = i, w = word);
    }

    // Create string with capacity

    let mut s = String::with_capacity(10);

    s.push('A');
    s.push('B');

    println!("[Strings.rs] {}", s);

    // Assertion testing
    
    assert_eq!(2, s.len());
    assert_eq!(16, capacity);
}