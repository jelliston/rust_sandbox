/**
 * Some command line interface arguments
 */

// Bring in standard library to shorten some of the code below
use std::env;

pub fn run() {
    // Whatever you add at the end of the 'cargo run' command gets collected in a vector
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let command = args[1].clone();
    let name = "Janky";
    let status = "100%";
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status: {}",status);
    } else {
        println!("That is not a valid command");
    }
}