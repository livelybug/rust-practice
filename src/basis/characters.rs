
fn string_test() {
    println!("string_test --------------------");
    // utf-8, on stack
    let s:&'static str = "hello world"; // &str = string slice
    for c in s.chars() {
        print!("{} ", c);  // h e l l o   w o r l d
    }
    println!(" ");

    if let Some(first) = s.chars().nth(0) {
        println!("First character : {}", first);
    }

    // String on heap
    let mut letters = String::new();
    let mut a = 'a' as u8; // define a char
    while a <= ('g' as u8) {
        letters.push(a as char); // push char
        letters.push_str("-"); // push string
        a += 1;
    }
    println!("letters : {}", letters);

    // convertion &str <> String
    let u:&str = &letters;
    let mut newstr = "hello".to_string();
    newstr.remove(0);
}

fn str_format_test() {
    println!("str_format_test ----------------");

    let name = "Jim";
    let greeting = format!("Hi, I'm {}", name);
    println!("{}", greeting);

    let format_idx = format!("{0} {1}, {0} {2}", "the", "good", "bad");
    println!("{}", format_idx);

    let format_ele_name = format!("I'm {first}, {first} {last}",
                                  first = "John", last = "Doe");
    println!("{}", format_ele_name);

    let format_mix = format!("{1} {} {0} {} {value}", "a", "b", value = "c");
    println!("{}", format_mix);
}

pub fn characters() {
    str_format_test();
    string_test();
}