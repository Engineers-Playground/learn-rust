// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display
fn display(a: bool) -> String{
    match a{
        true => "it's true".to_string(),
        false => "it's false".to_string()
    }
}
fn main() {
println!("{}", display(true));
}
