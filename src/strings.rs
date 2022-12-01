// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut hi = String::from("Hii");

    println!("{}", hello);
    println!("{}", hi);

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hi.push('y');
    println!("{}", hi);

    // Push string
    hi.push_str(" there");
    println!("{}", hi);

    // Capacity in bytes
    println!("Capacity: {}", hi.capacity());

    // Check if empty
    println!("{}", hi.is_empty());

    // Contains
    println!("Contains 'there' {}", hi.contains("there"));

    // Replace
    println!("Replace: {}", hi.replace("there", "world"));

    // Loop through string by whitespace
    for word in hi.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}