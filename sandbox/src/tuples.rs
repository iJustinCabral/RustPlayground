// Tuples group together values from different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Justin", "RI", 35);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}

