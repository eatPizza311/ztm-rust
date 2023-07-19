// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// Use an enum to create different flavors of drinks
enum Flavor {
    Orange,
    Banana,
    Watermelon,
}

// Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    volumn: f64,
}

// Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Orange => println!("Orange flavor!"),
        Flavor::Banana => println!("Banana flavor!"),
        Flavor::Watermelon => println!("Watermelon flavor!"),
    }
    println!("The volumn is {:?} oz.", drink.volumn)
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Banana,
        volumn: 10.6,
    };

    print_drink(my_drink);
}
