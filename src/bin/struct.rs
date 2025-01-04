#![allow(unused)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32,f32,f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main(){
    let p = Point {x:1.0, y:2.0};

    println!("value1 {} , value2 {}", p.x, p.y);

    let p = Point3d(1.0,2.0,3.0);

    println!("{} {} {}", p.0,p.1,p.2);

    let c = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 1,
    };

    println!("{:#?}", c);


    let x = 1.0;
    let y = 2.0;

    let p = Point {x,y};
   
    let p0 = Point{ x: 1.0, y: 1.0};
    let p1 = Point{ x: 1.0, ..p0};

    println!("{:?}", p1);

    let mut p = Point {x:0.0,y:0.0};
    p.x +=1.0;
    p.y +=2.0;

    println!("{:?}", p);


}