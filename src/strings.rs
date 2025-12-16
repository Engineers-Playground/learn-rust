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

struct Person {
    name: String,
    age: i32,
    color: String,
    bio: Option<String>
}
impl Person {
    fn print_info(&self) {
        println!("{} likes {}", self.name, self.color);
    }
}
fn main() {
let p1 = Person{
    name: "Domirando".to_owned(),
    age: 19,
    color: "Silver grey".to_owned(),
    bio: None
};
let p2 = Person{
    name: "Domi".to_owned(),
    age: 10,
    color: "grey".to_owned(),
    bio: Some(String::from("someone"))
};
let p3 = Person{
    name: "Do".to_owned(),
    age: 9,
    color: "Silver blue".to_owned(),
    bio: None
};
let people = vec![p1, p2, p3];
for person in people {
    if person.age <= 10 {
        person.print_info();
    }
}
}
