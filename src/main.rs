use serde_derive::Deserialize;
use serde_json;
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Theme {
    category: String,
    title: String,
    themes: Vec<String>,
}

fn usage() {
    println!("Usage: cargo run -- [<category> ...]");
    println!("category: [general | faces]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string("assets/inktoberthemes.json").expect("File doesn't exist.");
    let parsed = serde_json::from_str::<Vec<Theme>>(&contents).expect("JSON was not well formed");

    if args.len() == 1 {
        usage();
        return;
    }

    // for (i, a) in args.iter().enumerate() {
    //     println!("{}, {}", i, a);
    // }

    let categories = &args[1..];
    for category in categories {
        let mut themes = parsed
            .to_vec()
            .into_iter()
            .filter(|theme| &theme.category == category)
            .flat_map(|t| t.themes)
            .collect::<Vec<String>>();

        // remove duplicates
        // sort + dedup
        // source : https://www.reddit.com/r/rust/comments/38zzbk/best_way_to_remove_duplicates_from_a_veclist/
        themes.sort();
        themes.dedup_by(|a, b| a == b);

        if themes.len() == 0 {
            eprintln!("no themes linked to the category {}", category);
            continue;
        }

        let r: usize = rand::random();
        println!("{}: {}", category, themes[r % themes.len()]);
    }
}
