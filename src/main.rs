use std::env;
use std::fs;

fn main() {
    // args() returns an iterator
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should have to read file");
    println!("With text:\n{}", content);
}
