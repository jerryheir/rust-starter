pub fn run(){
    let arr1 = [1,2,3];
    let arr2 = arr1;

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Array Values: {:?}", (arr1, arr2));
    println!("Vector Values: {:?}", (&vec1, vec2));
}