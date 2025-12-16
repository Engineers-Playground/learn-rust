// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student{
    name: String,
    locker: Option<i32>
}

impl Student{
    fn st_locker(&self) {
        match self.locker{
            Some(ans) => println!("{}", self.locker.expect("reason").to_string()),
            None => println!("no assignment")
        }
    }
}

fn main() {
let st = Student{
    name: "maftuna".to_owned(),
    locker: Some(2)
};
st.st_locker();
}
