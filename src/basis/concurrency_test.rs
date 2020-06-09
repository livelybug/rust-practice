use std::sync::mpsc;
use std::thread;

fn channel_test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    /*
    match received {
        Ok(str) => println!("Got: {}", str),
        Err(e) => eprintln!("Err receiving : {:?}", e)
    }
    */
}

pub fn concurrency_test() {
    println!("concurrency_test----------------------");
    channel_test();
}