// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language (variables pertain to the scope in which they were defined)
// Convention for variables in Rust is to use underscores _  you don't use camelCase

pub fn run() {
    let name = "Ketchup";
    let mut age = 27;
    println!("My name is {} and I am {}", name, age);
    age = 28;

    println!("My name is {} and I am {}", name, age);

    // Define constants (need to explicitly define a type)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let ( my_name, my_age ) = ("Ketchup", 27);

    println!("{} is {}", my_name, my_age);
}