

// Javascript object style
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point {x: 3.2, y:4.3};
    println!("point at ({}, {})", p.x, p.y);
}

enum Color
{
    Red, Green, Blue,
    RGBColor(u8, u8, u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

fn enums(c:Color) {
    println!("enums test --------------------------");
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

union IntOrFloat
{
    i: i32,
    f: f32
}

fn union_test(mut iof:IntOrFloat){
    println!("union_test ---------------------");
    let value = unsafe {iof.i};
//    let value = iof.i;
    println!("iof.i = {}", value);

    unsafe {
        match iof {
            IntOrFloat {i:42} => println!("value is 42"),
            IntOrFloat {ref f} => println!("float is {}", f) // "ref" means pass by reference instead of by value
        }
    }
}

fn option_test(x:f32, y:f32) {
    println!("option_test -----------------");
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

fn array_test() {  // fixed size
    println!("array test ----------------------");
    let mut a:[i32;5] = [1,2,3,4,5];
    let mut b = [1,2,3,4,5];
    a[0] = 322;
    println!("{:?}", a);  // rust style loop, not seen in other languages!
    if a != [1,2,3,4,5] {
        println!("not equal [1,2,3,4,5]") // not seen in other languages!
    }

    let mut b = [1u16; 10]; // "1" is initial value, "u16" is type casting
    println!("{:?}", b);  //[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

    let c:[[u16; 3]; 2] = [
        [1,2,3],
        [4,5,6]
    ];
    println!("{:?}", c);

    for i in 0..c.len() {
        for j in 0..c[i].len() {
            print!("c[{}, {}] = {}; ", i, j, c[i][j]);
        }
    }
}

fn slice_test(slice: &mut[i16]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 198;
}

fn sum_product(x:u16, y:u16) -> (u16, f32) {
    (x + y, 1.0 * 3.2)
}

// struct tuple
struct Colour(u8, u8, u8);

fn tuple_test() {
    println!("tuple test ---------------------------");
    let x = 3;
    let y = 4;
    let mut sp = {(x + y, x as f32 * y as f32 )};
    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    // destructure
    let (a, b) = sp;

    let sp2 = sum_product(2, 7);
    let combined = (sp, sp2);
    println!("last element is {}", (combined.1).1); //nested tuple

    let single_ele_tpl = (32, ); //single element tuple, python similarity

    let red = Colour(255, 0, 0);
    println!("red is {}, {}, {}", red.0, red.1, red.2);
}

struct Point_g<T, V> {
    x: T,
    y: V
}

struct Line_g<T,V> {
    start: Point_g<T,V>,
    end: Point_g<T,V>
}

fn generic_test() {
    let a:Point_g<i32, i16> = Point_g{x:2, y:4};
    let b = Point_g{x:2, y:4};

    let line_c = Line_g{start: a, end: b};
}

pub fn data_structure() {
    generic_test();
    tuple_test();
    let mut data = [1,2,3,4,5];
    slice_test(&mut data[1..4]);
    println!("{:?}", data); // [1, 198, 3, 4, 5]
    slice_test(&mut data);
    println!("{:?}", data); // [198, 198, 3, 4, 5]

    array_test();
    let mut x = 3.0;
    let mut y = 0.0;
    option_test(x, y);
    y = 2.0;
    option_test(x, y);

    let mut iof = IntOrFloat{i:42};
    union_test(iof);
    iof.i = 22;
    union_test(iof); //float is 0.000000000000000000000000000000000000000000031

    let c:Color = Color::Red;
    enums(c);
    let c1:Color = Color::RGBColor(1,2,3);
    enums(c1);
    let c2:Color = Color::Cmyk{cyan:0, magenta:123, yellow:2, black:0};
    enums(c2);

    structures();
}