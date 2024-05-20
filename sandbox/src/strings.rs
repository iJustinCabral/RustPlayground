// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - USE WHEN EED TO MODIFY OR OWN STRING DATA

pub fn run() {
    // Immutable
    let hello = "Hello";

    // Growable
    let mut world = String::from("World");

    // Add to string
    world.push_str("!");

    // Capacity in bytes
    println!("Capacity: {}", world.capacity());

    // Check if string is empty
    println!("Is Empty: {}", world.is_empty());

    // Contains substring
    println!("Contains 'World' {}", world.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("Hello", "Hola"));

    // Loop through string by whitespace
    let long_word = "Hello from Mars!";
    for word in long_word.split_whitespace() {
        println!("{}", word);
    }

    // Create string with a capacity
    let mut s = String::with_capacity(10);
    s.push_str("0123456789");

    // Assertion Testing (Checks our string meets the capcity limit)
    assert_eq!(10, s.capacity());
    println!("{}", s);

    println!("Length: {}", hello.len());
    println!("{} {}", hello, world);
}
