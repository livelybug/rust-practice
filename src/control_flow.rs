#![allow(dead_code)]
use std::mem;
use std::io::stdin;

fn if_statement(temp:i32){
    // Functional programming style
    let day = if temp < 20 {"cold"} else if temp > 32 { "hot" } else { "good" }; // functional programming style
    println!("It's a {} day", day);
    println!("It's a {} day",
        if temp < 20 {
            if temp < 0 {"very cold"} else {"cold"}
        } else {"OK"}
    );
}

fn for_loop()
{
    println!("for_loop ---------------");

    for x in 1..8 {
        println!("x = {}", x);
    }
    for (pos, y) in (1..).enumerate() { // .. means infinite
        if(y > 8) { break; }
        println!("y = {}, pos = {}", y, pos);
    }
}

fn match_statement(countrycode:i32){
    let country = match countrycode{
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        z @ 1...999 => "unknown",  // 3 dots is obsolet now, replaced by "..="
        _ => "invalid"
    };
    println!("the country is {}", country);
}

fn conbination_lock() {
    enum State {
        Locked, Unlocked
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

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
}

pub fn control_flow() {
    conbination_lock();
    match_statement(44);
    match_statement(1);
    if_statement(-2);
    for_loop();
}