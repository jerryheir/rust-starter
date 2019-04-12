pub fn run(){
    let _hello = "orld!"; // primitive string
    let mut hey = String::from("Hello "); // growable string

    println!("The Length of {} is {}", hey, hey.len());

    hey.push('W');
    hey.push_str(_hello);
    println!("{}", hey);
    println!("Capacity: {}", hey.capacity());
    println!("Is Empty: {}", hey.is_empty());
    println!("Does it contain 'World' ?: {}", hey.contains("World"));
    println!("Replace 'World' {}", hey.replace("World", "There"));

    for word in hey.split_whitespace() {
        println!("{}", word);
    }

    assert_eq!(12, hey.len());

}