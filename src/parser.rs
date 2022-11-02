use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Theme {
    colors: HashMap<String, String>,
    space: Vec<i32>,
    font_sizes: Vec<i32>,
    fonts: HashMap<String, String>,
    font_weights: HashMap<String, i32>,
    line_heights: HashMap<String, f32>,
    breakpoints: HashMap<String, String>,
    animation: HashMap<String, String>,
    gradients: HashMap<String, String>,
}

pub fn typed_example(json_data: &str) -> Result<()> {
    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Theme = serde_json::from_str(json_data)?;

    // Do things just like with any other Rust data structure.
    println!(
        "Black Color: {} \n Smallest Font Size: {}",
        p.colors.get("black").unwrap(),
        p.font_sizes[0]
    );

    // Loop over all the colors
    for (key, color) in p.colors {
        println!("{}: {}", key, color);
    }

    Ok(())
}
