/*
Primitive str = immutable fixed-length string somewhere in memory
String = growable, heap-allocated data structure (use this when you need to modify or own string data)
*/


pub fn run() {
    
    let hello = "Hello";  //this is the primitive string
    let mut hi = String::from("Hi "); //this is the growable type of String, need to declare as mutable if want to change

    println!("{} {}", hello, hi);

    hi.push('J'); // 'push' method is only for character literals (1 codepoint)
    hi.push_str("ameson");  // use "push_str" for more than 1 codepoint 

    // Get length
    println!("{} Length: {}", hi, hi.len());

    // Capacity (in bytes that it can store)
    println!("Capacity: {}", hi.capacity());

    // Check if string is empty
    println!("Is empty: {}", hi.is_empty());

    // Check if contains some substring
    println!("Contains 'son': {}", hi.contains("son"));

    // Replace
    println!("Replace: {}", hi.replace("me", "ck"));

    // Loop through string by whitespace
    for word in hi.split_whitespace() {   // often see 'token' used instead of 'word'
        println!("{}", word);
    }

    // Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    
    println!("{}", s);

    // Assertions (test if left is equal to right)
    assert_eq!(2, s.len());  // if the test passes, then you don't see anything; only see error if fails
    assert_eq!(10, s.capacity());
}