#![allow(unused)]

fn main() {
    //int 
    // -(2**(n-1) ~ 2**(n-1) - 1
    let i0: i8 = 0;  // - 2**(8-1) ~  2**(8-1) - 1   - 128 to 127
    let i5: isize = 1;


    // uint   smae 
    let u0 : u8 = 1;  // 0 ~ 2**8 -1 = 255
    let u5 : usize = 1; 

    // float

    let f0: f32 = 0.01;
    let f1: f64 = 0.01;

    //boolean

    let b : bool = true;

    // charectors

    let c: char = 'c';


    // type conversion 

    let i: i32 = 1;
    let u: u32 = i as u32;

    let x: u32 = u + (i as u32);

    // min max 

    let min_i : i32 = i32::MIN;
    let max_i : i32 = i32::MAX;

    println!("{min_i} {max_i}");

    let min_char : char = char::MIN;
    let max_char : char = char::MAX;

    println!("{min_char} {max_char}");

    // overflow 

    let mut u: u32 = u32::MAX;
    u += 1; 
    println!("oveflow:{u}")


}