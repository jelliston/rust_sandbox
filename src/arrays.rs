/* 
Fixed list where elements are the same data types
Should set data type and array length
*/

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // use debug trait {:?} to print out entire array
    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[0]);

    // can change values within arrays, but can't add onto them
    let mut mut_numbers: [i32; 5] = [1, 2, 3, 4, 5]; 

    //change value
    mut_numbers[2] = 20;
    println!("{:?}", mut_numbers);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack-allocated (so they take up memory)
    // here can also take away 'std::' (standard library) by bringing it in at the top of the file: 'use std::mem;'
    println!("This array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice / need to use debug trait since it is an array to print out
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}