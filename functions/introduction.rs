// Simple function with no parameters or return value
fn greet() {
    println!("Hello, world!");
}

// Function with parameters
fn add_numbers(a: i32, b: i32) {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
}

// Function with a return value
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn main() {
    greet(); // Call the greet function to print "Hello, world!"
    add_numbers(5, 7); // Call the add_numbers function with arguments 5 and 7
    let result = multiply(3, 4); // Call the multiply function and assign the result to a variable
    println!("The product is {}", result); // Print the result
}

/*
    the output is that:
    Hello, world!
    The sum of 5 and 7 is 12
    The product is 12
    
    Cargo.toml will not change.
*/
