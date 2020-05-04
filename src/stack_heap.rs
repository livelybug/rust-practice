#![allow(dead_code)]
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 1.2, y: 2.3}
}

pub fn stack_and_heap()
{
    let p1 = origin(); //stored in stack
    let p2 = Box::new(origin()); // stored in heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));  // 16 bytes means the length of two f64
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));  // 8 bytes means the size of pointer in 64bit system

    let p3 = *p2;
    println!("p3.x is {}", p3.x);
}