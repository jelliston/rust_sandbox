/**
 Functions are used to store blocks of code for re-use
 */

pub fn run() {
    greeting("Hi", "Ketchup");
    
    // print result of function directly
    println!("Result of add function: {}", add(10, 34));

    // Bind function values to variable
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure (able to use outside variables)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// want the function to return something, must indicate what type to return
fn add(n1: i32, n2: i32) -> i32 {
    // not using the semi-colon indicates <return this>
    n1 + n2
}