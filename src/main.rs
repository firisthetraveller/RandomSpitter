#[macro_use]
extern crate json;

use rand::Rng;

fn main() {
    let json_file_path = Path::new("assets/inktoberthemes.json");
    let file = File::open(json_file_path);

    println!("Hello, world!");
}
