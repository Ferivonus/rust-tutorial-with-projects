// advanced.rs

fn main() {
    // Advanced Level: Variable Shadowing and Reassignment

    // Declare a variable named 'x' and assign a closure to it
    let x = || {
        // Define variables 'y' and 'z' within the closure
        let y = 5;
        let z = 10;

        // Return the sum of 'y' and 'z' from the closure
        y + z
    };

    // Call the closure stored in 'x' and assign the result to a new variable named 'x'
    let x_result = x();

    // Print the value of 'x_result' after calling the closure
    println!("The value of x_result after calling the closure is: {}", x_result);
}

/*
    The output is:
    The value of x_result after calling the closure is: 15

    Cargo.toml:

    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]


*/
