// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    // Add on to vector
    numbers.push(6);
    numbers.push(7);
    
    // Pop off the last value
    numbers.pop();

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get vector length
    println!("Vector length {}", numbers.len());

    // Vectors are stack allocated
    //println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}