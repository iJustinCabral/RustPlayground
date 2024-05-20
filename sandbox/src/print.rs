pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

   // Rust Uses String literals when printing variables to the console
    println!("Number {}", 1);
    println!("{} is from {}", "Justin", "Rhode Island");

    // Positional Agruments
    println!("{0} is from {1} and {0} likes to {2}", "Justin", "RI", "code");

    // Named Arguments:
    println!("{name} likes to {activity}", name = "Justin", activity = "Code");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10 ,10);

    // Placerholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

}
