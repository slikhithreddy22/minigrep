use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("searching for query {}", query);
    println!("in file_path, {}", file_path);

    let contents = fs::read_to_string(file_path).expect("should have able to read");
    println!("with context: \n {contents}");
}
