// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
enum Items {
    Chairs,
    Beds,
    Tables,
    Couches,
};
struct Item {
    name: Items,
    num: i32,
}
impl Item {
    fn item(&self) -> Result<Item, String> {
        if &self.num > 0{
            Ok(&self)
        else {
            Err("not left".to_owned)
        }    }
fn main() {
    let i1 = Item{
        name: Items.Chairs,
        num: 5
    };
let i2 = Item{
        name: Items.Beds,
        num: 3
    };
let i3 = Item{
        name: Items.Beds,
        num: 3
    };
let mut stock = HashMap::new();
}
