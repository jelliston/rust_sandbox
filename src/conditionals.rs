/*
Conditionals are used to check the condition of something and act on the result
*/

pub fn run() {

    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true; 

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to be 18 to drink here."); 
    } else {
        println!("Bartender: I'll need to see your ID.");  
    }

    // Shorthand if statement
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age); 

}