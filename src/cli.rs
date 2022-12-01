use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Semih";
    let status = "100%";

    //println!("Args: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }

    //h(command);
}

pub fn h(x: String) -> Result<i32, String> {
    if x == "hello" {
        Ok(100)
    }else {
        Err(String::from("Error"))
    }
}
