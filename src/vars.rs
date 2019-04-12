pub fn run(){
    let name = "Jerry";
    let mut age = 25;
    println!("My name is {}, and I am {}", name, age);

    age = 26;
    println!("My name is {}, and I am {}", name, age);

    // define constant 
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign mulitple vars
    let (my_name, age, so_sha) = ("Jeremiah", 27, "so!!!!");
    println!("{} is {} years old {}", my_name, age, so_sha);
}