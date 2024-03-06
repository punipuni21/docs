use std::{io, result};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let result = stdin.read_line(&mut buffer).ok();

    match result {
        Some(n) => {
            println!("{}", buffer);
        }
        None => println!("error: unable to read user input"),
    }
}
