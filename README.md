[TOC]
# How to run
* run
```commandline
cargo run
```
* key in ```1234``` , then enter

# Rust Language Basis
## Type and Variables
### Core Data Type
```rust
let mut a:u8 = 123; // 0 .. 256
let b:i8 = 123; //-128 .. 127
let z:isize = 123; // isize/usize, OS bit length
let size_of_z = mem::size_of_val(&z);
println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z*8); 
//z = 123, takes up 8 bytes, 64-bit os
```

### Global variables
```rust
//Global, no fixed address in memory. because they’re inlined to each place they’re used.
const MEANING_A:u8 = 42;  
//Global, not inlined upon use
static mut Z:i32 = 323;  

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
```

### Stack & Heap
```rust
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 1.2, y: 2.3}
}

pub fn stack_and_heap()
{
    let p1 = origin(); //stored in stack
    let p2 = Box::new(origin()); // stored in heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));  // 16 bytes means the length of two f64
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));  // 8 bytes means the size of pointer in 64bit system
}
```

## Control Flow
### For Loop
```rust
for x in 1..8 { // 1..8 is type of iterator
    println!("x = {}", x);
}
for (pos, y) in (1..).enumerate() { // .. means infinite
    if(y > 8) { break; }
    println!("y = {}, pos = {}", y, pos);
}
```
### If statement
```rust
// Scala stypel functional programming style
let day = 
    if temp < 20 {"cold"}  // no "return" keyword needed 
    else if temp > 32 { "hot" } 
    else { "good" }; 
```
### Match statement
```rust
fn match_statement(countrycode:i32){
    let country = match countrycode{
        44 => "UK",
        7 => "Russia",
        z @ 1..=999 => "unknown",  // "z" is reusable
        _ => "invalid"
    };
    println!("the country is {}", country);
}
```
### loop match
```rust
loop {
    match state {
        State::Locked => {
            let mut input = String::new();
            match stdin().read_line(& mut input) {
                Ok(_) => { entry.push_str(&input.trim_end()); }
                Err(_) => continue
            }
            if entry == code {
                state = State::Unlocked;
                continue;
            }
            if !code.starts_with(&entry) {
                entry.clear();
                println!("Failed！")
            }
        }
        State::Unlocked => {
            println!("Unlocked!");
            return;
        }
    }
}
```
## Data Structure
### Generic
```rust
struct Point_g<T, V> {
    x: T,
    y: V
}
let a:Point_g<i32, i16> = Point_g{x:2, y:4};
let b = Point_g{x:2, y:4}; // implicit type, java 7 feature
```
### Tuple
```rust
let x = 3;
let y = 4;
let mut sp = {(x + y, x as f32 * y as f32 )};
println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
// destructure
let (a, b) = sp;
// nested tuple
let combined = (sp, sp);
println!("last element is {}", (combined.1).1); //nested tuple
//single element tuple, python similarity
let single_ele_tpl = (32, ); 
```
### Slice / Partial Array
```rust
fn slice_test(slice: &mut[i16]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 198;
}
let mut data = [1,2,3,4,5];
slice_test(&mut data[1..4]);
println!("{:?}", data); // [1, 198, 3, 4, 5]
slice_test(&mut data);
println!("{:?}", data); // [198, 198, 3, 4, 5]
```

### Array - fixed size
```rust
let mut a:[i32;5] = [1,2,3,4,5];
a[0] = 322;
println!("{:?}", a);  // rust style debug, not seen in other languages!
if a != [1,2,3,4,5] { // not seen in other languages!
    println!("not equal [1,2,3,4,5]") 
}
let mut b = [1u16; 10]; // "1" is initial value, "u16" is type
println!("{:?}", b);  //[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
let c:[[u16; 3]; 2] = [
    [1,2,3],
    [4,5,6]
];
```
### Option
```rust
fn option_test(x:f32, y:f32) {
    let result =
        if y != 0.0 {Some(x/y)} else {None};
    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("Cannot devide by 0.0")
    }
    if let Some(z) = result {  // compare option
        println!("result = {}", z)
    }
}
option_test(2.0, 3.0);
```
### Union
```rust
union IntOrFloat{
    i: i32,
    f: f32
}

fn union_test(mut iof:IntOrFloat){
    let value = unsafe {iof.i};
    println!("iof.i = {}", value);

    unsafe {  // unsafe due to the data type maybe wrong
        match iof {
            IntOrFloat {i:42} => println!("value is 42"),
            IntOrFloat {ref f} => println!("float is {}", f) // "ref" means pass by reference instead of by value
        }
    }
}

iof.i = 22;
// find float on the address of an int 
union_test(iof); //float is 0.000000000000000000000000000000000000000000031
```

### Enums
```rust
enum Color
{
    Red, Green, Blue,
    RGBColor(u8, u8, u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

fn enums(c:Color) {
    match c {
        Color::Red => println!("color is red"),
        Color::Green => println!("color is green"),
        Color::RGBColor(0,0,0) => println!("black"),
        Color::RGBColor(r,g,b) => println!("r,g,b({}, {}, {})", r, g, b),
        Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255} => println!("color is black"),
        Color::Cmyk{black:255, ..} => println!("color is black"),  // means the same as the line above
        _ => println!("unkown color"),
    }
}
let c1:Color = Color::RGBColor(1,2,3);
enums(c1); // r,g,b(1, 2, 3)
let c2:Color = Color::Cmyk{cyan:0, magenta:123, yellow:2, black:0};
enums(c2); // unkown color
```
## Standard Collection
### Vector (ArrayList in Java)
```rust
let mut a = Vec::new();
a.push(3);
a.push(4);

//    let idx:i32 = 0; // not work in rust!!! singed value not allowed, and value must match the OS bit length
let idx:usize = 0;
println!("a[0] = {}", a[idx]);
println!("a.get(1) is {:?}", a.get(1));  //a.get(1) is Some(4)

while let Some(x) = a.pop() {  // Iterate vector
    println!("{}", x);
}
```
### HashMap
```rust
let mut shapes = HashMap::new();
shapes.insert(String::from("triangle"), 3);
shapes.insert(String::from("square"), 4);
// the 2 lines below are equal
println!("a square has {} sides", shapes["square".into()]);  // The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, Into will call it when necessary.
println!("a square has {} sides", shapes[&String::from("square")]);
for (k, v) in &shapes {
    println!("key - {} : value - {}", k, v);
}

shapes.entry("circle".into()).or_insert(1);
{
    let actual = shapes.entry("circle".into()).or_insert(2);
    *actual = 0;
}
println!("shapes: {:?}", shapes);  // shapes: {"circle": 0, "square": 4, "triangle": 3}
```

### HashSet
```rust
let mut set1 = HashSet::new();
set1.insert("game");
set1.insert("desk");
let _2_5: HashSet<_> = (2..=5).collect();
let _1_6: HashSet<_> = (1..=6).collect();
```

## Functions
### Function as variable
```rush
fn say_hi() {println!("Hi");}
let sh = say_hi;
sh();

let mut one = 1;
let plus_one = |x:i32| -> i32 {x + 1};
let plus_one_1 = |x| {x + one};  // implicit type
```
### Pattern & Reference of Function parameter
* [rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=323f896c95e428fcf2d67b431d54f125)
```rust
fn print_tuple(t: (i32, i32)) { // no param pattern or reference
    println!("Current location: ({}, {})", t.0, t.1);
}
fn print_tuple_pat((x, y): (i32, i32)) { //  pattern param, no reference
    println!("Current location: ({}, {})", x, y);
}
fn print_tuple_ref(t: &(i32, i32)) {  // no param pattern, wiht reference
    println!("Current location: ({}, {})", (*t).0, (*t).1);
}
fn print_tuple_ref_val(&t: &(i32, i32)) {  // reference of type and value
    println!("Current location: ({}, {})", t.0, t.1);
}
fn print_tuple_ref_pat(&(x, y): &(i32, i32)) {  // param pattern, reference of type and value
    println!("Current location: ({}, {})", x, y);
}
fn param_pattern_reference() {
    print_tuple((1,2));
    print_tuple_pat((1,2));
    print_tuple_ref(&(1, 2));
    print_tuple_ref_val(&(1, 2));
    print_tuple_ref_pat(&(1,2));
}

```

## Trait
### Trait Basis
* Inheritance
```rust
trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human { Human{name: name} }
    fn name(&self) -> &'static str { self.name }
}

let john = Human{name: "John"};
let john2 = Human::create("John2");
let john3:Human = Animal::create("John3");
```
### Trait Parameter
* 3 ways to set parameter type
```rust
fn print_name(obj: impl Animal + Debug) {
    println!("{:?}'s name is {}", obj, obj.name());
}
fn print_name2<T:Animal + Debug>(obj: T) {
    println!("{:?}'s name is {}", obj, obj.name());
}
fn print_name3<T>(obj: T) where T: Animal + Debug {
    println!("{:?}'s name is {}", obj, obj.name());
}
fn trait_parameter() {
    let animal = Human{name: "John4"};
    print_name(animal);
    let animal2 = Human{name: "John5"};
    print_name2(animal2);
    let animal3 = Human{name: "John6"};
    print_name2(animal3);
}
```
### Trait - Into
```rust
struct HumanStr {
    name: String
}
impl HumanStr {
    fn new<S: Into<String>>(name: S) -> HumanStr { // Into<T> means the type is convertable to T
        HumanStr{name: name.into()}
    }
}
fn into_test() {
    let animal_into = HumanStr::new("John_Into");
}
```
### Trait - Drop
* Similar to C++ destructor
```rust
impl Drop for HumanStr {
    fn drop(&mut self) {
        println!("{} is cleared", self.name)
    }
}
fn drop_test() { // destrctor
    let animal3 = HumanStr::new2("John3");
    drop(animal3);  //Optional, if not called explicitly, it's called auto at the end scope
}
```
### Trait - Override Operator
```rust
use std::ops::{Add, AddAssign};
use std::cmp::PartialEq;

#[derive(PartialEq)] // using standard instead of overriding  
#[derive(Debug)]
struct Complex<T> {
    le: T,
    wd: T
}
impl<T> Complex<T> {
    fn new(le:T, wd:T) -> Complex<T> {
        Complex::<T> {le, wd}  // :: is to avoid compiler to treat < and less operator
    }
}
// without - where T: Add<Output = T> - error: cannot add `T` to `T`,  `T` might need a bound for `std::ops::Add`
impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            le: self.le + rhs.le,
            wd: self.wd + rhs.wd
        }
    }
}
impl<T> AddAssign for Complex<T> where T:AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.le += rhs.le;
        self.wd += rhs.wd;
    }
}

fn operator_overload() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(2, 4);
    println!("addition = {:?}", a + b);
    let mut a2 = Complex::new(1, 2);
    let mut b2 = Complex::new(2, 4);
    a2 += b2;
    println!("addassign = {:?}", a2);
}
```
### Static Dispatch
* Static Dispatch / add method to a Type
```rust
trait Summable<T> {
    fn sum(&self) -> T;
}

// Static Dispatch - add method to a Type
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {result += *x}
        result
    }
}
fn print_dispatch_static(disp: Vec<i32>) {
    println!("static dispatch sum is {}", disp.sum());
}

let a = vec![1,2,3];
print_dispatch_static(a);

```
### Dynamic Dispatch
* Knowing the size of the value at compile time is important for things like passing it as an argument to a function, moving it about on the stack and allocating (and deallocating) space on the heap to store it. 
```rust
//fn print_dispatch_dynamic(disp: Summable<i32>) {
// The line above caused error: doesn't have a size known at compile-time

fn print_dispatch_dynamic(disp: &Summable<i32>) {
    println!("dynamic dispatch sum is {}", disp.sum());
}
let a = vec![1,2,3];
print_dispatch_dynamic(&a);
```

## Variable Access
### Ownership
```rust
let v1 = vec![1,2,3];
// error: borrow of moved value: `v1`, as ownership of the vector is transferred to v2
// let v2 = v1;
// println!("v1 = {:?}", v1);

let int1 = 1;
let int2 = int1;
// no error, as for primitive type, the value is copied automatically, no ownership transfer
println!("int1 = {}", int1);

let return_ownership = |x:Vec<i32>| -> Vec<i32> {
    println!("return_ownership value = {:?}", x);
    x
};
let v3 = return_ownership(v1);
println!("v3 = {:?}", v3); // ownership transferred to v3
```
### Borrowing
```rust
    let v1 = vec![1,2,3];
//    let my_print = |x:Vec<i32>| {println!("my print {:?}", x)}; // error: borrow of moved value
    let my_print = |x:&Vec<i32>| {println!("my print {:?}", *x)};  // Ampersand means borrowing the vector for a while
    my_print(&v1);
    println!("v1 = {:?}", v1);

//    cannot borrow `z` as mutable because it is also borrowed as immutable
//    let mut z = vec![1,2,3];
//    for i in &z {
//        println!("i = {}", i);
//        z.push(i + 1);
//    }
```

### Lifetime specifier
```rust
struct Person {
    name: String
}
impl Person {
    // fn get_name<'a>(&'a self) -> &'a String  // compiler convert the function signature implicitly
    fn get_name(&self) -> &String {
        &self.name
    }
}
// principal: &Person
//              ^ expected named lifetime parameter
struct School<'z> {
    name: String,
    principal: &'z Person
}
fn lifetime_test() {
    let teacher1 = Person{name: String::from("John")};
    let school1 = School{name: String::from("School1"), principal: &teacher1};
}
```
### Lifetime structure
```rust
struct Person1<'a> {
    name:&'a str
}
impl<'a> Person1<'a> { // A struct has lifetime specifier, its trait needs lifetime too
    fn say_hi(&self) {
        println!("Hi, I am {}", self.name);
    }
}
fn lifetime_structure() {
    let person1 = Person1{name: "John"};
    person1.say_hi();
}

```

### Reference Counting
```rust
use std::rc::Rc; // Limited to one single thread
struct Vectest {
    name: Rc<Vec<i32>>
}
fn reference_count() { // cause some of the compiler effort. When a variable is simply used through ownership and borrowing, you have static compiler checks that you're not introducing a data race
    let v1 = Rc::new( vec![1,2,3]);
    println!("1: v1 has {} strong pointers", Rc::strong_count(&v1));
    {
        let vectest1 = Vectest { name: v1.clone() }; //Rc::clone doesn’t make a deep copy, only increments the reference count
        println!("2: v1 has {} strong pointers", Rc::strong_count(&v1));
    }
    println!("3: v1 has {} strong pointers", Rc::strong_count(&v1));  // 2nd reference cout GC as out of closure
}

```
```
1: v1 has 1 strong pointers
2: v1 has 2 strong pointers
3: v1 has 1 strong pointers
```

### Atomic Rererce Counting - Thread Safe
```rust
use std::sync::{Arc,Mutex};
use std::thread;
#[derive(Debug)]
struct VectestThread {
    name: Arc<Vec<i32>>
}

fn atomic_ref_count() {
    let v1 = Arc::new( vec![1,2,3]);
    println!("1: v1 has {} strong pointers", Arc::strong_count(&v1));
    let vectest1 = VectestThread{name: v1.clone()};
    let v2 = v1.clone();
    let thread1 = thread::spawn(move || {
        println!("vectest1 = {:?}", vectest1);
        println!("vectest1 = {:?}", v2);
        println!("2: v1 has {} strong pointers", Arc::strong_count(&v2));
    });
    println!("3: v1 has {} strong pointers", Arc::strong_count(&v1));
    thread1.join().unwrap();
    println!("4: v1 has {} strong pointers", Arc::strong_count(&v1));
}
```
```
1: v1 has 1 strong pointers
3: v1 has 3 strong pointers
vectest1 = VectestThread { name: [1, 2, 3] }
vectest1 = [1, 2, 3]
2: v1 has 3 strong pointers
4: v1 has 1 strong pointers

```

### Atomic Mutex Reference Count
```rust
let mut str1 = Arc::new(Mutex::new("str1".to_string()));
println!("str1 is {:?}", str1);
let mut str2 =str1.clone();
let thread1 = thread::spawn(move || {
    let mut str3 = str2.lock().unwrap();
    str3.clear();
    println!("str3 is {:?}", str3);
});
thread1.join().unwrap();
println!("str1 is {:?}", str1);
```
```
str1 is Mutex { data: "str1" }
str3 is ""
str1 is Mutex { data: "" }
```
## Misc
### Use 3rd party Crate
* Cargo.toml
```toml
[dependencies]
rand = "0.7"
```

```rust
extern crate rand;
use miscl::rand::prelude::*;

let x:u8 = random();
let y:bool = random();
println!("x = {}, y = {}", x, y);
```

### Create Crate
* Create crate structure
```bash
rust-practice/my_crate$ tree
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── lib.rs
    └── talk
        ├── cat.rs
        └── dog.rs
```
* Build the crate 
```bash
rust-practice/my_crate$ cargo build
```
* Cargo.toml
```toml
[dependencies]
my_crate = { path = "my_crate" }
```

* Invoke the crate
```rust
extern crate my_crate;
use miscl::my_crate::talk::cat;
use miscl::my_crate::talk::dog;

println!("{}", cat::hello());
println!("{}", dog::hello());
```
