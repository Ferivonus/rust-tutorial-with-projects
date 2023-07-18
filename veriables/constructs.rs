// Custom struct representing a person
struct Person {
    name: String,
    age: u32,
    height: u32,
}

impl Person {
    // Constructor for Person struct
    fn new(name: &str, age: u32, height: u32) -> Person {
        Person {
            name: String::from(name),
            age,
            height,
        }
    }

    // Method to get the person's name
    fn get_name(&self) -> &str {
        &self.name
    }

    // Method to get the person's age
    fn get_age(&self) -> u32 {
        self.age
    }
    
    fn get_height(&self) -> u32 {
        self.height
    }
}

// Enum representing different status options
enum Status {
    Active,
    Inactive, // will be never used. 
    Pending,
}

fn main() {
    // Creating a new instance of Person
    let person = Person::new("ferivon", 24, 187);

    // Accessing the person's name and age
    println!("Person's name: {}", person.get_name());
    println!("Person's age: {}", person.get_age());
    println!("Person's heigh: {}", person.get_height());

    // Using an enum in a match expression
    let mut status = Status::Active;
    match status {
        Status::Active => println!("User is active."),
        Status::Inactive => println!("User is inactive."),
        Status::Pending => println!("User is pending approval."),
    }
    status = Status::Pending;

    match status {
        Status::Active => println!("User is active."),
        Status::Inactive => println!("User is inactive."),
        Status::Pending => println!("User is pending approval."),
    }
}

/*
    The output of that code is that:
    Person's name: ferivon
    Person's age: 24
    Person's heigh: 187
    User is active.
    User is pending approval.
*/