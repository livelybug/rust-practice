extern crate reqwest;
use futures::executor::block_on;
use reqwest::Client;

async fn hello_world() {
    println!("hello, async rust!");
}

async fn http_request() {
    let client = Client::new();
    let resp = client.get("https://httpbin.org/ip").send().await;
    match resp {
        Ok(b) =>  println!("Got {:?}", b),
        Err(e) => eprintln!("Http request got error： {:?}", e)
    }
}

pub fn async_rust() {
    println!("async_rust ---------------");
    // block_on(http_request());  //unable to restrict with timeout due to compilation error of futures 0.3.5
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}