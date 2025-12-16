// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
let mut a = 5;
loop{
    if a > 1{
        a-=1;
    }else if a < 1{
        a+=1;
    }else {
        println!("4");
        break;
    }
}
}
