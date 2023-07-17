// Function with default parameter value
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with variable number of arguments
fn calculate_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

// Function that returns multiple values using a tuple
fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
    let min = numbers.iter().min().unwrap_or(&0);
    let max = numbers.iter().max().unwrap_or(&0);
    let average = calculate_sum(numbers) as f64 / numbers.len() as f64;
    (min.clone(), max.clone(), average)
}

fn main() {
    greet("fer"); // Call greet function with the name "fer"
    greet("ivon"); // Call greet function with the name "ivon"

    let numbers = [1, 2, 3, 4, 5];
    let sum = calculate_sum(&numbers); // Call calculate_sum function with the array of numbers
    println!("Sum: {}", sum);

    let (min, max, average) = calculate_stats(&numbers); // Call calculate_stats function with the array of numbers
    println!("Min: {}, Max: {}, Average: {:.2}", min, max, average);
}

/*

    The output is that:
    Hello, fer!
    Hello, ivon!
    Sum: 15
    Min: 1, Max: 5, Average: 3.00

    Cargo.toml will not change.

*/