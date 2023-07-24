// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let my_string_lower = "hello";
    println!(
        "Lower case string {:?} can be convert to upper case using .to_uppercase {:?}",
        my_string_lower,
        my_string_lower.to_uppercase()
    );
    let my_string_upper = "HELLO";
    println!(
        "Upper case string {:?} can be convert to lower case using .to_lowercase {:?}",
        my_string_upper,
        my_string_upper.to_lowercase()
    )
}
