use std::env;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.nya>", args[0]);
        exit(1);
    }
    let path = Path::new(args[1].as_str());
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", path.display(), err),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}", path.display(), err),
        Ok(_) => interpreter(s),
    };
}

fn interpreter(code: String) {
    let mut i: i32 = 0;
    for c in code.chars() {
        match c {
            'n' => i -= 1,
            'y' => i += 1,
            'a' => print!("{}", i as u8 as char),
            '~' => i = 0,
            _ => continue,
        };
    }
    println!();
}
