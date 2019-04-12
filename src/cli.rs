use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Jerry";
    let status = "100%";

    // println!("Args: {:?}", args);
    // println!("Command: {}", command);
    if args[1].is_exists() && command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if args[1].is_exists() && command == "status" {
        println!("Status is {}", status);
    } else {
        println!("This is not a valid command")
    }
}