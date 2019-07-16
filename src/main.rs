#[warn(unused_imports)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    println!("Searching for {}", query);

    for result in BufReader::new(File::open(query)?).lines() {
        let l = result?;
        println!("{}", l);
    }
    Ok(())
}
