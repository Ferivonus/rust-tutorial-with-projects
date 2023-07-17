
fn main() {
    // String Manipulations

    // Declare a mutable string variable named 'greeting' and assign it the value "Hello"
    let mut greeting = String::from("Hello");

    // Append ", world!" to the 'greeting' string
    greeting.push_str(", world!");

    // Replace the substring "Hello" in the 'greeting' string with "Hi there"
    greeting = greeting.replace("Hello", "Hi there");

    // Split the 'greeting' string into individual words
    let words: Vec<&str> = greeting.split_whitespace().collect();

    // Print the modified greeting
    println!("Modified greeting: {}", greeting);

    // Print individual words
    println!("Individual words:");
    for word in &words {
        println!("{}", word);
    }

    // Check if the greeting contains the word "Hi"
    let contains_hi = greeting.contains("Hi");
    println!("Does the greeting contain 'Hi'? {}", contains_hi);
}

/*
    The output is that:
    Modified greeting: Hi there, world!
    Individual words:
    Hi
    there,
    world!
    Does the greeting contain 'Hi'? true

    Cargo.toml:

    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    
    [dependencies]
*/
