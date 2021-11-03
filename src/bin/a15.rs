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

// * Use an enum for the tickets with data associated with each variant
enum Ticket{
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
// * Use a match expression while iterating the vector to print the ticket info
fn main() {
    let owner= vec![
        Ticket::Backstage(50.0, "kakuru".to_string()),
        Ticket::Standard(15.0),
        Ticket::Vip(70.0, "pat".to_string()),
    ];
    for ticket in owner{
        match ticket{
            Ticket::Backstage(price, holder) => println!("holder: {:?}, price: {:?}", price, holder),
            Ticket::Standard(price) => println!(" price: {:?}", price),
            Ticket::Vip(price, holder) => println!("holder: {:?}, price: {:?}", price, holder),
        }
    }
}
