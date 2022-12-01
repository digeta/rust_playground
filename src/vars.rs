// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Semih";
    let mut age = 35;

    println!("My name is {} and I was {} 5 years ago", name, age);

    age = 39;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Semih", 39);
    println!("{} is {}", my_name, my_age);
}