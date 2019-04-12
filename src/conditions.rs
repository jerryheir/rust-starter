pub fn run(){
    let age: u8 = 10;
    let check_id: bool = true;
    let knows_person_of_age = false;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Teacher: What is your name?");
    } else if age < 21 && check_id {
        println!("Teacher: Welcome to the Rust class.");
    } else {
        println!("Teacher: Where are you from?");
    };

    // short hand if/else
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}