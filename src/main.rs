use serde_derive::Deserialize;
use serde_json;
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Themes {
    category: String,
    title: String,
    themes: Vec<String>,
}

fn usage() {
    println!("Usage: cargo run -- <category>");
    println!("category: [general | faces]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string("assets/inktoberthemes.json").expect("File doesn't exist.");
    let parsed = serde_json::from_str::<Vec<Themes>>(&contents).expect("JSON was not well formed");

    if args.len() == 1 {
        usage();
        return;
    }

    // println!("{:?}", args);

    let category = &args[1];
    let themes: Vec<String> = parsed
        .into_iter()
        .filter(|theme| &theme.category == category)
        .flat_map(|t| t.themes)
        .collect::<Vec<String>>();

    let r: usize = rand::random();
    println!("{}", themes[r % themes.len()]);
}
