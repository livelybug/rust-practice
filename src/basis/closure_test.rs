use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn closure_() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));  // type of x is set to String after this line
    // let n = example_closure(5);
    // 5 |     let n = example_closure(5);
    // |                             ^
    //     |                             expected struct `std::string::String`, found integer

//closures have an additional capability that functions don’t have:
// they can capture their environment and access variables from the scope in which they’re defined.
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

//    functions are never allowed to capture their environment,
//     fn equal_to_x(z: i32) -> bool {
//         z == x
//     }
//     assert!(equal_to_x(y));
    // 5 |         z == x
    //     |              ^
    //     |
    //     = help: use the `|| { ... }` closure form instead
}

fn shared_state() {
    println!("shared_state -------------------");
    let counter = Arc::new(Mutex::new(0));
    let mut handles:Vec<JoinHandle<()>> = vec![];
    // let mut handles= vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("num = {:?}", num);
        });
        handles.push(handle);
    }

    for handle in handles {
        println!("handle = {:?}", handle);
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn closure_test() {
    shared_state();
    closure_();
}