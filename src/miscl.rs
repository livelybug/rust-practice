extern crate rand;
extern crate my_crate;
use rand::prelude::*;
use my_crate::talk::cat;
use my_crate::talk::dog;

fn code_block() {
    let x = 10;
    {
        let y = 9;
    }
    // no access to y outside the code block
}

fn shadowing_test() {
    let x = 10;
    let x  = "hello".to_string();
    // no compiling error, variable's data type can be changed
}


pub fn miscl() {
    println!("miscl ----------------------");
    code_block();
    let x:u8 = random();
    let y:bool = random();
    println!("x = {}, y = {}", x, y);

    println!("my_crate ------------------");
    println!("{}", cat::hello());
    println!("{}", dog::hello());

}