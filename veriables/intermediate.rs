// intermediate.rs

fn main() {
    // Intermediate Level: Mutable Variables and Type Annotation

    // Declare a mutable variable named 'counter' of type 'u32' with an initial value of a function call
    let mut counter: u32 = get_initial_counter();

    // Print the initial value of 'counter'
    println!("The initial value of counter is: {}", counter);

    // Call a function to modify the value of 'counter'
    modify_counter(&mut counter);

    // Print the updated value of 'counter'
    println!("The updated value of counter is: {}", counter);
}

fn get_initial_counter() -> u32 {
    // Simulate a complex computation to obtain the initial value
    42
}

fn modify_counter(counter: &mut u32) {
    // Modify the value of 'counter' by adding 10
    *counter += 10;
}
