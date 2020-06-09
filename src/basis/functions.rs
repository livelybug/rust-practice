
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn get_radius(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}

fn struct_method() {
    println!("struct_method -------------------");
    let p = Point{x:3.2, y:1.2};
    println!("radisu = {}", p.get_radius());

}

fn say_hi() {println!("Hi");}

fn closure_func_var() {
    println!("closure_func_var -------------------");
    let sh = say_hi;
    sh();

    let one = 1;
    let plus_one = |x:i32| -> i32 {x + 1};
    let plus_one_1 = |x| {x + one};  // implicit type
    println!("{} + 1 = {}", 3, plus_one_1(3));
    let borrow_one = &one;
    println!("{} + 1 = {}", 4, plus_one_1(4));
}

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
    println!("param_pattern_reference -------------");
    print_tuple((1,2));
    print_tuple_pat((1,2));
    print_tuple_ref(&(1, 2));
    print_tuple_ref_val(&(1, 2));
    print_tuple_ref_pat(&(1,2));
}

fn higher_order_function() {
    let limit = 200;
    let mut sum = 0;
    for i in 0.. {
        let isq = i*i;
        if isq > limit {break}
        else if isq % 2 == 0 {sum += isq;}
    }

    let sum2 =
        (0..).map(|x| x*x)
            .take_while(|&x|  x < limit)
            .filter(|&x| (x %2 == 0))
            .fold(0, |sum, x| sum+x);
    println!("sum2 is {}", sum2);
}

pub fn functions() {
    higher_order_function();
    param_pattern_reference();
    closure_func_var();
    struct_method();
}