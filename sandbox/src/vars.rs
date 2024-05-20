// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped Language

pub fn run() {
    let name = "Justin";
    
    // Need to add mut to make the variable mutatable
    let mut age = 35;   

    println!("My name is {} and I am {}", name, age);

    // Define constant (Requires type decleration)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple Values
    let (my_name, my_age) = ("Justin", 35);
    println!("{} is {}", my_name, my_age);
}
