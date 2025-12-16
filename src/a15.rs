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
enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}

fn main() {
let t: Vec<Ticket> = vec![Ticket::Backstage(31, "owner".to_owned()), Ticket::Standard(23), Ticket::Vip(12, "owner2".to_owned())]; 
for i in t{
match i {
    Ticket::Backstage(price, holder) => println!("backstage: holder: {:?}, price: {:?}", holder, price),
 Ticket::Vip(price, holder) => println!("vip: holder: {:?}, price: {:?}", holder, price),
Ticket::Standard(price) => println!("standard: price: {:?}", price)
}}}
