use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename)?;
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    for line in io::BufReader::new(file).lines() {
        let l = line?;
        lines += 1;
        words += l.split(' ').count();
        chars += l.chars().count();
    }
    println!("lines: {}", lines);
    println!("words: {}", words);
    println!("chars: {}", chars);
        
    Ok(())
}
