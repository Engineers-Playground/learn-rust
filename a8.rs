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
enum Flavor {
    Sweet,
    Juice
}
struct Drink{
    flavor: Flavor,
    ounce: f64
}
fn main() {
let drink = Drink{
    flavor: Flavor::Sweet,
    ounce: 0.5
};
match drink.flavor{
    Flavor::Sweet => print!("sweet"),
    Flavor::Juice => print!("juice")
};

println!(" - {}", drink.ounce);
}
