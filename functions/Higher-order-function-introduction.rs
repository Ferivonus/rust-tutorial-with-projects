fn apply_function(value: i32, func: fn(i32) -> i32) -> i32 {
    func(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn double(value: i32) -> i32 {
    value * 2
}

fn main() {
    let result = apply_function(5, square); // Call apply_function with the square function
    println!("Result: {}", result);

    let result = apply_function(7, double); // Call apply_function with the double function
    println!("Result: {}", result);
}

/*
    Output of this code:

    Result: 25
    Result: 14
*/