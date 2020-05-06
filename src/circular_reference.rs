//! The major function `circular_reference()` is not a demo of how to use circular reference `RefCell`.
//! Instead, the code demonstrates how to prevent using `RefCell`.
//! The approach is introducing data redundancies for many to many relationship.
//! # Example
//! ```
//!     let john = Student{name: "John".into()};
//!     let course1 = Course{name: "Rust Course".into()};
//!     let enroll1 = Enrollment{course: &course1, student: &john};
//!     let mut platform1 = Platform::new("platform1".into());
//!     platform1.add(enroll1);
//! ```

struct Student {
    name: String
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    course: &'a Course,
    student: &'a Student
}

impl<'a> Enrollment<'a> {
    fn new(course: &'a Course, student: &'a Student) -> Enrollment<'a> {
        Enrollment{course, student}
    }
}

struct Platform<'a> {
    name: String,
    enrolls: Vec<Enrollment<'a>>
}

/// Online course platform such as cousera, udemy...
impl<'a> Platform<'a> {
    fn new(name: String) -> Platform<'a> {
        Platform{name, enrolls: vec![] }
    }

    fn add(&mut self, enroll: Enrollment<'a>) {
        self.enrolls.push(enroll);
    }
}

impl Student {
    fn get_courses(&self, platform: Platform) -> Vec<String> {
        platform.enrolls.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}
/// Create a student `john`;
/// Create a course `Rust Course`;
/// `john` enroll in the course `Rust Course`;
/// The enrollment happens on platform `platform1`;
pub fn circular_reference() {
    println!("circular_reference ---------------------");
    let john = Student{name: "John".into()};
    let course1 = Course{name: "Rust Course".into()};
    let enroll1 = Enrollment{course: &course1, student: &john};
    let mut platform1 = Platform::new("platform1".into());
    platform1.add(enroll1);
    println!("John is taking:\n {:?} ", john.get_courses(platform1));
}