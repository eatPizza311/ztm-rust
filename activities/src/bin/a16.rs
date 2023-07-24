// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// Use a struct containing the student's name and locker assignment
// The locker assignment should use an Option<i32>
struct Locker {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let lockers = vec![
        Locker {
            name: "Amy".to_owned(),
            locker: Some(32),
        },
        Locker {
            name: String::from("Geoge"),
            locker: None,
        },
    ];
    for lk in lockers {
        println!("Student: {:?}", lk.name);
        match lk.locker {
            Some(num) => println!("Locker number is {:?}", num),
            None => println!("No locker assigned!"),
        }
    }
}
