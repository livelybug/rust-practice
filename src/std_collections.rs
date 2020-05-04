use std::collections::HashMap;
use std::collections::HashSet;

fn vector_test() {
    println!("vector_test -------------------");
    let mut a = Vec::new();
    a.push(3);
    a.push(4);

//    let idx:i32 = 0; // not work in rust!!! singed value not allowed, and value must match the OS bit length
    let idx:usize = 0;
    println!("a[0] = {}", a[idx]);
    println!("a.get(1) is {:?}", a.get(1));  //a.get(1) is Some(4)
    match a.get(1) {  // vector methods return option type
        Some(x) => println!("a[1] = {}", x),
        None => println!("No such element")
    }

    while let Some(x) = a.pop() {  // Iterate vector
        println!("{}", x);
    }
}

fn hashmap_test() {
    println!("hash_map test ----------------------");
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);
    println!("a square has {} sides", shapes[&String::from("square")]);
    for (k, v) in &shapes {
        println!("key - {} : value - {}", k, v);
    }

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("shapes: {:?}", shapes);
}

fn hashset_test() {
    let mut set1 = HashSet::new();
    set1.insert("game");
    set1.insert("desk");
    println!("{:?}", set1);

    let _2_5: HashSet<_> = (2..=5).collect();
    let _1_6: HashSet<_> = (1..=6).collect();

    println!("is {:?} a subset of {:?}? {}", _2_5, _1_6,
        _2_5.is_subset(&_1_6));
    // disjoint = no common elements
    println!("Does {:?} have commone elements with {:?}? {}", _2_5, _1_6,
             _2_5.is_disjoint(&_1_6));
}

pub fn std_collect() {
    hashset_test();
    hashmap_test();
    vector_test();
}