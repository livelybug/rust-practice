
fn ownership() {
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
}

fn borrowing() {
    println!("borrowing test --------------------");
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
}
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

struct Person1<'a> {
    name:&'a str
}

impl<'a> Person1<'a> {
    fn say_hi(&self) {
        println!("Hi, I am {}", self.name);
    }
}

fn lifetime_structure() {
    let person1 = Person1{name: "John"};
    person1.say_hi();
}

use std::rc::Rc; // Limited to one single thread

struct Vectest {
    name: Rc<Vec<i32>>
}

fn reference_counted() { // lose some of the compiler guarantees. When a variable is simply used through ownership and borrowing, you have static compiler checks that you're not introducing a data race
    println!("reference_counting ------------------------");
    let v1 = Rc::new( vec![1,2,3]);
    println!("1: v1 has {} strong pointers", Rc::strong_count(&v1));
    {
        let vectest1 = Vectest { name: v1.clone() }; //Rc::clone doesn’t make a deep copy, only increments the reference count
        println!("2: v1 has {} strong pointers", Rc::strong_count(&v1));
    }
//    println!("v1 = {:?}", v1);
    println!("3: v1 has {} strong pointers", Rc::strong_count(&v1));
}

use std::sync::{Arc,Mutex};
use std::thread;
#[derive(Debug)]
struct VectestThread {
    name: Arc<Vec<i32>>
}

fn atomic_ref_count() {
    println!("atomic_ref_count ------------------");
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

fn mutex_thread_safe() {
    println!("mutex_thread_safe -----------------");
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
}

pub fn variable_access() {
    mutex_thread_safe();
    atomic_ref_count();
    reference_counted();
    lifetime_test();
    borrowing();
    ownership();
}