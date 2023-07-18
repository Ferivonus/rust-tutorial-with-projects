// Define a simple custom macro called `greet`
macro_rules! greet {
    // Define a pattern that matches when the macro is invoked without any arguments
    () => {
        println!("Hello, world!");
    };
    
    // Define a pattern that matches when the macro is invoked with a single argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    
    // Define a pattern that matches when the macro is invoked with multiple arguments
    ($name:expr, $greeting:expr) => {
        println!("{} {}", $greeting, $name);
    };
}

fn main() {
    greet!(); // Invoke the `greet` macro without any arguments
    greet!("feri"); // Invoke the `greet` macro with a single argument
    greet!("von", "Greetings :3"); // Invoke the `greet` macro with multiple arguments
}
/*

    the output of this code:

    You can easily understand that.

*/
