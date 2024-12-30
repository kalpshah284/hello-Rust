#![allow(ususe)]

// tuple is the datatype

fn main() {
    // tuple
    let t: (bool, u32, char) = (true, 1, 'c');
    //destructure
    let (a, b, c) = t;
    //ingore value
    let (_, b, _) = t;

    // emty touple - uint type
    let t = ();

    // nested tuple 

    let nested = ((1.2, 'a'), (true, 1u32, 'b'), ());

    println!("nested {},{}", nested.0.0, nested.1.1);
}