// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// * Use a function to display your first name
fn firstName(first_name: String) -> String {
    println!("{first_name}");
    first_name
}
// * Use a function to display your last name
fn lastName(last_name: String) -> String {
 
    last_name
}

fn main()  {
   firstName("maftuna".to_string());
   println!("{}", lastName("pulatova".to_string()));

}

