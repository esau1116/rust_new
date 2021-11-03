// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student{
    name:Option<String>,
    locker:Option<i32>,
}
fn main() {
    let kaks = Student{
        name:Some(String::from("Jacob")),
        locker:Some(24),
    };
    match kaks.name{
        Some(ans) => println!("name: {:?}", ans),
        None => println!("name: no response")
    };
    match kaks.locker{
        Some(ans) => println!("locker: {:?}", ans),
        None => println!("none"),
    };

}
