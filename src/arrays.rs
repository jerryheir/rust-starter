use std::mem;

pub fn run(){
    let num: [i32; 5] = [1, 2, 3, 4, 5];

    let mut numbers: [i32; 6] = [6, 90, 01, 18, 9, 90];

    numbers[1] = 22;

    println!("{:?}", num);

    println!("Single value: {}", num[0]);

    println!("Updated numbers array: {:?}, and its length is {}", numbers, numbers.len());

    println!("The num array occupies {} bytes and the other occupies {}", std::mem::size_of_val(&num), mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}",slice);
}