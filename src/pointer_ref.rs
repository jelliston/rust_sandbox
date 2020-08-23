/**
 Reference pointers - point to a reference in memory

 With non-primitives, if you assign another variable to a piece of data, the first variable will
 no-longer hold that value. You'll need to use a reference (&) to point to the resource
 */

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // Vectors (non-primitive); need to point to a reference
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;  

    println!("Vec values: {:?}", (&vec1, vec2));    
}