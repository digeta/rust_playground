pub fn run() {
    //let cars = ["mercedes", "porsche", "audi", "lamborghini", "maserati"];
    /*
    let cars: [i32; 5] = [1,2,3,4,5];

    println!("Cars: {:?}", cars);

    for car in cars {
        println!("Car: {}", car);
    }

    for car in cars {
        println!("The Car: {}", car);
    }

    let x = 5;
    let y = x;
    println!("X: {}", x);
    println!("Y: {}", y);

    let s1 = String::from("hello");
    let n1: i32 = 124;

    println!("s1: {}", s1);
    let s2 = s1;
    println!("s2: {}", s2);
    println!("n1: {}", n1);

    let str1: &str = "semih";
    let str2 = str1;
    println!("str1: {}", str1);
    println!("str2: {}", str2);
    */

    let n = 124;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };

    let description_match = match is_even(n) {
        true => "even",
        false => "odd"
    };

    println!("{} is {}", n, description);
    println!("{} is {}", n, description_match);

    let haystack: [i32; 7] = [1, 1, 2, 5, 42, 132, 429];

    for needle in haystack {
        let result = match needle {
            42 | 132 => "found it!",
            _ => "miss"
        };
        println!("{}: {}", needle, result);
    }

    let r = &haystack;

    println!("Size of haystack array: {}", std::mem::size_of_val(&haystack));
    println!("Size of haystack's reference: {}", std::mem::size_of_val(r));

    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + r = {}", b);


    /* */

    let mut sx = String::from("helloyyy");
    let mut sy = String::from("");

    {
        let rx1 = &mut sx;
        rx1.push_str("worllld");
        sy.push_str(rx1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let rx2 = &mut sx;
    println!("sx: {}", sx);
    println!("sy: {}", sy);
}

fn is_even(n: i32) -> bool
{
if n % 2 == 0 {
    true
} else {
    false
}
}