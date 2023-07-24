// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// Use an enum for the tickets with data associated with each variant
// Tickets can be Backstage, Vip, and Standard
// Backstage and Vip tickets include the ticket holder's name
// All tickets include the price
enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main() {
    // Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(100, "Justin".to_owned()),
        Ticket::Vip(80, "Danny".to_owned()),
        Ticket::Standard(50),
    ];

    // Use a match expression while iterating the vector to print the ticket info
    for tk in tickets {
        match tk {
            Ticket::Backstage(price, holder) => {
                println!("Holder: {:?}, Price: {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("Holder: {:?}, Price: {:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Price: {:?}", price),
        }
    }
}
