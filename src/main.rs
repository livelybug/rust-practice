#![allow(dead_code)]
#![allow(unused_variables)]
mod stack_heap;
mod control_flow;
mod data_structure;
mod std_collections;
mod characters;
mod functions;
mod traits;
mod variable_access;
mod circular_reference;
mod miscl;
use std::mem;


const MEANING_A:u8 = 42;  //Global, no fixed address in memory. because they’re inlined to each place they’re used.
static mut Z:i32 = 323;  //Global, aren’t inlined upon use

fn operators()
{
    let mut a = 2+3*4;
    println!("{}", a);

    a -=2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{}", a_cubed);

    let b =2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);
}

fn scope_and_shadow()
{
    let a = 123;
    let a = 122;

    {
        let b = 456;
        println!("inside b = {}\n", b);
        let a = 938;
        println!("inside a = {}\n", a);

    }
    println!("outside a = {}\n", a);
}

fn global_var()
{
    println!("= {}", MEANING_A);
    println!("= {}", unsafe { Z });

    unsafe
    {
        Z = 4343;
        println!("= {}", Z);
    }
}

fn main() {
    miscl::miscl();
    circular_reference::circular_reference();
    variable_access::variable_access();
    traits::traits();
    functions::functions();
    characters::characters();
    std_collections::std_collect();
    data_structure::data_structure();
    control_flow::control_flow();
    stack_heap::stack_and_heap();
    global_var();
    scope_and_shadow();

    let a:u8 = 123; // 0 .. 256
    let b:i8 = 123; //-128 .. 127
    println!("a = {}", a);
    println!("Hello, world!");

    let mut c = 12;
    println!("c = {}", c);
    c = 1334534353;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, after modifying size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z*8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.3;  // double precision
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    operators();
}
