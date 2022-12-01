pub fn run() {
    let age: u8 = 22;
    let check_id = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Barteder: What would you like to drink?");
    }
    else if age < 21 && check_id{
        println!("Barteder: Sorry you have to leave");
    }
    else {
        println!("Barteder: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is Of Age {}", is_of_age);
}