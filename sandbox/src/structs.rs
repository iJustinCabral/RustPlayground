// Traditional Struct
struct Color {
    red:    u8,
    green:  u8,
    blue:   u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

// Associated functions with struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set lest name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}


pub fn run() {
    let mut c = Color { red: 255, green: 0, blue: 0 };
    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorTuple(200,100,50);
    ct.1 = 150;
    println!("Color Tuple: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("Justin", "Cabral");
    println!("Person: {}", p.full_name());
    p.set_last_name("Cabralllllll");
    println!("Person: {}", p.full_name());

}
