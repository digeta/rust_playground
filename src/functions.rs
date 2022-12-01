// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeding("Hello", "Elena");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure (outside variables)
    let m3: i32 = 10;
    let add_nums = |m1: i32, m2: i32| m1 + m2 + m3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeding(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}