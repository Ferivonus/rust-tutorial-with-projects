fn main() {
    // Integer variables
    let num1: i32 = 10;
    let num2: i32 = 20;
    let sum = num1 + num2;
    println!("Sum: {}", sum);

    // Floating-point variables
    let pi: f64 = 3.14159;
    let radius: f64 = 5.0;
    let area = pi * radius * radius;
    println!("Area: {}", area);

    // Boolean variables
    let is_greater = num1 > num2;
    println!("Is num1 greater than num2? {}", is_greater);

    // Character variables
    let letter: char = 'A';
    let unicode_value = letter as u32;
    println!("Letter: {}, Unicode value: {}", letter, unicode_value);

    // String variables
    let name1 = "feriv".to_string();
    let name2 = String::from("onus");
    let full_name = format!("{} {}", name1, name2);
    println!("Full name: {}", full_name);

    // Array variables
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Numbers: {:?}", numbers);

    // Tuple variables
    let person: (String, i32, bool) = ("feriv".to_string(), 24, true);
    println!("Person: {:?}", person);

    // Slices
    let message = "Hello, world!";
    let slice = &message[7..12];
    println!("Slice: {}", slice);

    // Option variables
    let some_value: Option<i32> = Some(42);
    let _none_value: Option<i32> = None;
    match some_value {
        Some(value) => println!("Some value: {}", value),
        None => println!("No value"),
    }

    // Result variables
    let result: Result<i32, String> = Ok(42);
    match result {
        Ok(value) => println!("Result value: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
/*
    Output of this code:

    Sum: 30
    Area: 78.53975
    Is num1 greater than num2? false
    Letter: A, Unicode value: 65
    Full name: feriv onus
    Numbers: [1, 2, 3]
    Person: ("feriv", 24, true)
    Slice: world
    Some value: 42
    Result value: 42

*/