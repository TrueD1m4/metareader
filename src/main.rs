mod arguments;

use std::{env, process};
use std::fs;
use arguments::Arguments;

fn main() {
    let argues: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&argues).unwrap_or_else(
        |err| {
            match err.as_str() {
                "Help" => {
                    println!("Help");
                    process::exit(0);
                },
                "Invalid syntax!" => {
                    eprintln!("Check help!");
                    process::exit(0);
                },
                _ => {
                    eprintln!("Problem with parsing arguments: {}", err);
                    process::exit(0);
                }
            }
        }
    );

    let source_file = fs::read_to_string(arguments.filepath.clone())
        .expect("File doesn't exists");
    println!("{:?}", arguments);
}
