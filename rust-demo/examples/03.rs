use std::fmt::Display;

fn main() {
    let jp2 = "Jan Paweł drugi jest największym polakiem".to_string();
    let funny_number = 2137;
    // or: "...".to_owned()
    // or: String::from("...")
    // or: let a: String = "...".into()
    // choose your favourite

    borrow_pope(&funny_number);

    // means the same as: println!("from main: {}", jp2);
    println!("from main: {funny_number}");

    move_pope(funny_number);

    println!("from main: {funny_number}");
}

fn borrow_pope<T: Display>(pope: &T) {
    println!("borrowed pope says: {pope}");
}

fn move_pope<T: Display>(pope: T) {
    println!("moved pope says: {pope}");
}
