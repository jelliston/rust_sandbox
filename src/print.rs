pub fn run () {
    // Print to console
    println!("hello from the print.rs file!");

    // Basic formatting
    println!("{}",1);

    println!("{} is from {}", "joel", "california");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Joel", "California", "code");


    // Named arguments
    println!("{name} likes to play {activity}", name = "john", activity = "baseball");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits (tuple)
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}