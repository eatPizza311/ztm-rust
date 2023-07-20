// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// Use a struct for the grocery item
struct GroceryItem {
    // Use two i32 fields for the quantity and id number
    quantity: i32,
    id_number: i32,
}

// Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

// Create a function to display the id number, with the struct as a parameter
fn display_id_number(item: &GroceryItem) {
    println!("ID Number: {}", item.id_number);
}

fn main() {
    let my_item = GroceryItem {
        quantity: 10,
        id_number: 100,
    };
    display_quantity(&my_item);
    display_id_number(&my_item);
}
