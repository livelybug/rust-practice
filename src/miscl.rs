extern crate rand;
extern crate my_crate;
use miscl::rand::prelude::*;
use miscl::my_crate::talk::cat;
use miscl::my_crate::talk::dog;

pub fn miscl() {
    println!("miscl ----------------------");
    let x:u8 = random();
    let y:bool = random();
    println!("x = {}, y = {}", x, y);

    println!("my_crate ------------------");
    println!("{}", cat::hello());
    println!("{}", dog::hello());
}