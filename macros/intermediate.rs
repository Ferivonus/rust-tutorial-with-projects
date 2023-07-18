macro_rules! calculate {
    // Define a pattern that matches when the macro is invoked with an addition operation
    (addition $x:expr, $y:expr) => {
        $x + $y
    };
    
    // Define a pattern that matches when the macro is invoked with a subtraction operation
    (subtraction $x:expr, $y:expr) => {
        $x - $y
    };
    
    // Define a pattern that matches when the macro is invoked with a multiplication operation
    (multiplication $x:expr, $y:expr) => {
        $x * $y
    };
    
    // Define a pattern that matches when the macro is invoked with a division operation
    (division $x:expr, $y:expr) => {
        $x / $y
    };
}

fn main() {
    let result1 = calculate!(addition 5, 3); // Invoke the `calculate` macro for addition
    println!("Result of addition: {}", result1);

    let result2 = calculate!(subtraction 10, 7); // Invoke the `calculate` macro for subtraction
    println!("Result of subtraction: {}", result2);

    let result3 = calculate!(multiplication 4, 6); // Invoke the `calculate` macro for multiplication
    println!("Result of multiplication: {}", result3);

    let result4 = calculate!(division 15, 3); // Invoke the `calculate` macro for division
    println!("Result of division: {}", result4);
}
/*
    The output of this code:
    Result of addition: 8
    Result of subtraction: 3
    Result of multiplication: 24
    Result of division: 5

*/