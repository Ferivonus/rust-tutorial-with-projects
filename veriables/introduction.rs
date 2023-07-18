fn main() {
    // Integer variable with explicit type annotation
    let age: i32 = 24;
    println!("Age: {}", age);

    // Floating-point variable
    let temperature = 32.5;
    println!("Temperature: {}", temperature);

    // Boolean variable with explicit type annotation
    let is_nailing: bool = false;
    println!("Is it nailing? {}", is_nailing);

    // Character variable with explicit type annotation
    let grade: char = 'A';
    println!("Grade: {}", grade);

    // String variable
    let name: String = String::from("Ferivon");
    println!("Name: {}", name);
 
     // Variables are immutable by default, meaning their values cannot be changed once assigned.
     // Uncomment the following line to see a compilation error.
     // age = 25; // This will produce an error: "cannot assign twice to immutable variable `x`"
 
     // To declare a mutable variable, use the `mut` keyword.
     let mut y = 10; // `y` is a mutable variable of type `i32` with an initial value of 10.
     println!("The value of y is: {}", y);
 
     // But mutable variables can be reassigned.
     y = 15;
     println!("The new value of y is: {}", y);
}

/*
    The output is that:

    The initial value of counter is: 42
    The updated value of counter is: 52

    Cargo.toml:
    
    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]

*/


