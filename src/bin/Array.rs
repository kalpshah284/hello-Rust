#![allow(unused)]

fn main() {
    let arr: [u32; 3] = [1,2,3];

    println!("arr[2] = {}", arr[2]);

    let mut arr: [u32; 3] = [1, 2, 3];

    arr[1] = 1000;

    println!("arr[1] = {}", arr[1]);

    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);

    // Slice - lenth is not compile time 
    let nums: [i32; 7] = [-1,-2,-3,0,1,2,3];

    //first 3 element 
    let s = &nums[..3];

    println!("first 3 elements: {:?}", s);
    
    // middle elements

    let s = &nums[3..5];
    println!("middle 3 elements: {:?}", s);

    
    let s = &nums[5..];
    println!("last 3 elements: {:?}", s);

    let s = &nums[..];
    println!("all elements: {:?}", s);

    





}
