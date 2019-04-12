use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![6, 90, 01, 18, 9, 90];

    numbers[1] = 22;

    numbers.push(5);
    numbers.push(54);
    numbers.push(42);
    numbers.pop();


    println!("Updated numbers vector: {:?}, and its length is {}", numbers, numbers.len());

    println!("The numbers vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}",slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Number: {:?}", numbers);
}