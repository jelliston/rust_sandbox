/*
Vectors are re-sizable arrays
Only need to define type, not length
*/

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // use debug trait {:?} to print out entire array
    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[0]);

    //change value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    // Pop off numbers from the end
    numbers.pop();
    println!("{:?}", numbers);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack-allocated (so they take up memory)
    // here can also take away 'std::' (standard library) by bringing it in at the top of the file: 'use std::mem;'
    println!("This vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice / need to use debug trait since it is a vector to print out
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

}