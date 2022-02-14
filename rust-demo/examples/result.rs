fn main() {
    let text: String = std::fs::read_to_string("file.txt").unwrap();
    println!("{text}");
}
