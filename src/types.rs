pub fn run(){
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater: bool = 10 > 5;
    let is_lesser = 10 < 5;
    let a3 = 'a';
    let face = '\u{1F600}';

    println!("{:#?}", (is_active, is_greater, is_lesser, a3, face))
}