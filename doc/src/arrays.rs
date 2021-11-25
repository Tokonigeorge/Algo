//arrays here are a fixed list length where elements are of the same data type
//to use std::mem we can do;
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    //arrays here can't be pushed to but the values can be reassigned
    println!("{:?}", numbers);

    numbers[0]=2;

    println!("The first value is {}", numbers[0]);
    //length
    println!("The length is {}", numbers.len());
    //the memory
    println!("The sie of the array is {} bytes", mem::size_of_val(&numbers));
    //get slice of array
    let slice: &[i32] = &numbers;
    println!("This copies the arry {:?}", slice);
    //get slice of array from 0 to 2
    let second_slice: &[i32] = &numbers[0..2];
    println!("This is the second slice {:?}", second_slice);
}