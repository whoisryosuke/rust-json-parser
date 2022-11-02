use std::fs;

pub mod parser;

fn main() {
    println!("Hello, world!");

    // Grab JSON file
    let file_path = "data/theme.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    // Parse the JSON
    let result = parser::typed_example(&contents);

    // Handle errors from the parser if any
    match result {
        Ok(result) => (),
        Err(error) => print!("{}", error),
    }
}
