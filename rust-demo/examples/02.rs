fn main() {
    let jp2 = "Jan Paweł drugi jest największym polakiem".to_string();
    // or: "...".to_owned()
    // or: String::from("...")
    // or: let a: String = "...".into()
    // choose your favourite

    borrow_pope(&jp2);

    // means the same as: println!("from main: {}", jp2);
    println!("from main: {jp2}");

    move_pope(jp2);

    println!("from main: {jp2}");
}

fn borrow_pope(pope: &str) {
    println!("borrowed pope says: {pope}");
}

fn move_pope(pope: String) {
    println!("moved pope says: {pope}");
}
