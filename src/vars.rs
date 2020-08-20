// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language (variables pertain to the scope in which they were defined)

pub fn run() {
    let name = "Ketchup";
    let mut age = 27;
    println!("My name is {} and I am {}", name, age);
    age = 28;

    println!("My name is {} and I am {}", name, age);

    // Define constants (need to explicitly define a type)
    const ID: i32 = 001;
    println!("ID: {}", ID);
}