use std::env;

pub fn run() {
    match env::home_dir() {
        Some(path) => println!("This is how you get your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}