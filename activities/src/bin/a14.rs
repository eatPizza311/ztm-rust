// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // The color and name should be stored as a String
    fav_color: String,
    name: String,
}

fn print_fav(name: &str, color: &str) {
    println!("{:?} favorite color is {:?}", name, color)
}

fn main() {
    // Create and store at least 3 people in a vector
    let people: Vec<Person> = vec![
        Person {
            age: 15,
            fav_color: "Blue".to_owned(),
            name: "John".to_owned(),
        },
        Person {
            age: 10,
            fav_color: String::from("Pink"),
            name: String::from("Mary"),
        },
        Person {
            age: 6,
            fav_color: "Green".to_owned(),
            name: String::from("David"),
        },
    ];

    // Iterate through the vector using a for..in loop
    for person in people {
        // Use an if expression to determine which person's info should be printed
        // Print out the name and favorite colors of people aged 10 and under
        if person.age <= 10 {
            // The name and colors should be printed using a function
            print_fav(&person.name, &person.fav_color);
        }
    }
}
