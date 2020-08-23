/**
 * Similar to classes, used to create custom data types
 */

// Traditional struct for an rgb color
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Tuple_color(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String
}

// Functions associated with 'Person' struct
impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Create method that gets full name; self represents the struct of Person
    // format is a macro, similar to println, but it doesn't print
    // no semi-colon because returning something
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Trad struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple struct
    let mut tc = Tuple_color(255,0,0);
    tc.1 = 100;
    println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);

    // Struct with functions
    let mut p = Person::new("Ketchup", "Johnson");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Jackson");
    println!("Person {}", p.full_name());

    println!("Person tuple: {:?}", p.to_tuple());
}