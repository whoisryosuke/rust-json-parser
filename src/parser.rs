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

#[derive(Default)]
struct ThemeTokens {
    colors: HashMap<String, String>,
    space: HashMap<String, String>,
    font_sizes: HashMap<String, String>,
    fonts: HashMap<String, String>,
    font_weights: HashMap<String, String>,
    line_heights: HashMap<String, String>,
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

    // We store the CSS variables we create to pass back to user for theming across app
    // Initialize the theme tokens struct
    let mut theme_tokens: ThemeTokens = ThemeTokens::default();

    // Create a stylesheet to pass lines of CSS into
    // Custom properties go into `:root`, media queries are separate
    // Combined for simplicity now
    // but starts to require a struct here (or leaning on a CSS crate/parser)
    let mut stylesheet = Vec::new();

    // Loop over all the colors
    for (key, color) in p.colors {
        // Create the custom property
        let custom_property = format!("--colors-{}", key);
        let css_rule = format!("{}: {};", &custom_property, color);

        // @TODO: Export a CSS stylesheet file (or return CSS)
        println!("{}", css_rule);
        stylesheet.push(css_rule);

        // Add the custom property
        theme_tokens.colors.insert(key, custom_property);
    }

    println!(
        "Token colors.text is var({});",
        theme_tokens.colors.get("text").unwrap()
    );

    println!("Stylesheet");

    println!("{}", stylesheet.join("\n"));

    Ok(())
}
