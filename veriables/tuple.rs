fn main() {
    // Tuple Variables
    let person: (&str, usize, bool) = ("Alice", 30, true);

    // Destructuring the tuple
    let (name, age, is_active) = person;

    // Accessing individual elements
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", is_active);

    // Modifying the tuple
    let mut modified_person = person;
    modified_person.1 += 1;
    println!("Modified Age: {}", modified_person.1);
}

/*
    The output is that:

    Name: Alice
    Age: 30
    Active: true
    Modified Age: 31

    Cargo.toml:
    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
*/