pub fn bomb(){
    // Print to console
    println!("Hello from the print.rs file");
    // basic formatting
    println!("{} age is {} so {}", "Jeremiah's", 11+13, "shut up");
    // posiional arguments
    println!("{2} age is {0} so {1}", "Jeremiah's", 11+13, "shut up");
    // named arguments
    println!("{name} loves playing {game}", name="Jerry", game="Chess");
    // placeholder traits
    println!("Binary: {:b} Hex: {:x}, Octal: {:o},  TT", 10,10,10);
    // placeholder for debug trait
    println!("{:?} then {:#?}", (12, true, "hello"), (10, true, "hello"));
}