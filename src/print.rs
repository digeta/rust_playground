pub fn run()
{
    // Print to console
    println!("Hello from the print.rs file");

    println!("Number: {}",1);

    // Basic Formatting
    println!("{} is from {}", "Milk", "Cow");

    // Positional Arguments
    println!("{0} is from {1} and {0} like to {2}", "Semih", "Zonguldak", "Sex");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Semih", activity = "with his dick");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello")); //tuple

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}