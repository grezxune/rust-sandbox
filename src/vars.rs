// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Tommy";
    let mut age = 27;

    println!("My name is {} and I am {} years old", name, age);

    age = 28;
    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Tommy", 27);
    println!("My name: {}\nMy age:{}", my_name, my_age);
}