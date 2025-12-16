// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox{
    dimensions: i32,
    weight: f32,
    color: String
}

impl ShippingBox {
    fn print_box(&self){
        println!("dimensions: {}, weight: {}, color: {}", self.dimensions, self.weight, self.color);
    }
}


fn main() {
let s_box = ShippingBox {
    dimensions: 3,
    weight: 34.5,
    color: "red".to_string()
};
s_box.print_box();
}
