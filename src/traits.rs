use std::fmt::Debug;
use std::ops::{Add, AddAssign};
use std::cmp::PartialEq;

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

#[derive(Debug)]
struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human { Human{name: name} }
    fn name(&self) -> &'static str { self.name }
}

fn trait_test() {
    println!("trait_test ------------------");
    let john = Human{name: "John"};
    john.talk();
    let john2 = Human::create("John2");
    john2.talk();
    let john3:Human = Animal::create("John3");
    john3.talk();
}

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

struct HumanStr {
    name: String
}

impl HumanStr {
    fn new<S: Into<String>>(name: S) -> HumanStr { // Into<T> means the type is convertable to T
        HumanStr{name: name.into()}
    }
    fn new2<S>(name: S) -> HumanStr where S: Into<String> {
        HumanStr{name: name.into()}
    }
}

fn into_test() {
    let animal_into = HumanStr::new("John_Into");
    let animal_into2 = HumanStr::new2("John_Into".to_string());
}

impl Drop for HumanStr {
    fn drop(&mut self) {
        println!("{} is cleared", self.name)
    }
}

fn drop_test() { // destrctor
    let animal3 = HumanStr::new2("John3");
    drop(animal3);  //Optional, as it's called auto at the end scope
}

//#[derive(PartialEq)] // instead of implement your own
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

impl<T> PartialEq for Complex<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.le == other.le && self.wd == other.wd
    }

    fn ne(&self, other: &Self) -> bool {
        unimplemented!()
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

trait Summable<T> {
    fn sum(&self) -> T;
}

// add method to a Type
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

//fn print_dispatch_dynamic(disp: &Vec<i32>) {
fn print_dispatch_dynamic(disp: &Summable<i32>) {
    println!("dynamic dispatch sum is {}", disp.sum());
}

trait Shape {
    fn area(&self) -> f32;
}
struct Square {side: f32}
struct Circle {radius: f32}
impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.1415926
    }
}

fn static_dispatch() {
    println!("static_dispatch ----------------");
    let a = vec![1,2,3];
    print_dispatch_static(a);
}

fn dynamic_dispatch() {
    println!("dynamic_dispatch ----------------");
    let a = vec![1,2,3];
    print_dispatch_dynamic(&a);
    let s1 = Square{side: 2.1};
    let c1 = Circle{radius: 3.2};
    let shapes:[&Shape; 2] = [&s1, &c1];
    for shape in shapes.iter() {
        println!("{}", shape.area());
    }
}

enum Creature {
    Square(Square),
    Circle(Circle)
}

fn diff_type_in_vec() {
    // Enum approach
    let mut creatures = Vec::new();
    creatures.push(Creature::Square(
        Square{side: 4.2}
    ));
    creatures.push(Creature::Circle(
        Circle{radius: 1.2}
    ));
    for c in creatures {
        match c {
            Creature::Circle(c) => println!("circle area is {}", c.area()),
            Creature::Square(s) => println!("square area is {}", s.area())
        }
    }

    // Box approach
    let mut shapes2:Vec<Box<Shape>> = Vec::new();
    shapes2.push(Box::new(Square{side: 3.1}));
    shapes2.push(Box::new(Circle{radius: 3.1}));

}

pub fn traits() {
    diff_type_in_vec();
    dynamic_dispatch();
    static_dispatch();
    operator_overload();
    drop_test();
    into_test();
    trait_parameter();
    trait_test();
}