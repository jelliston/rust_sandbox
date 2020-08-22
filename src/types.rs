/* 
Primitive types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
/*
Rust is a statically typed language, which means that it must know the types of all variables
at compile time, however, the compiler can usually infer what type we want to use based on 
the value and how we use it. So it's not required to set the type for every single var.
*/


pub fn run() {
    // By default this will by a type of "i32"
    let x = 1;

    // By default this will be a "f64"
   let y = 2.5;

    // Add explicit type
    let z: i64 = 3453454534534534;

    // Find max size
    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    // Boolean
    let is_active = true;             // here type of boolean is inferred  
    let is_real_active: bool = true;  // here type is explicit

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Char - signified with single quotes
    let a1 = 'a'; // must be a single character (unicode)
    let face = '\u{1F600}'; // unicode smileyface  (google 'emoji unicode' to get list)


    println!("{:?}", (x, y, z, is_active, is_real_active, is_greater, a1, face));

}