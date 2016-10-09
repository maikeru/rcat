use std::error::Error;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Argument was {}", args[1]);

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Failed to open {} with error {}", display, why.description()),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("Couldn't read from file {} with error {}", display, why.description()),
        Ok(_) => print!("{} contains:\n {}", display, content),
    }

}
