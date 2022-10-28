use std::env;

/// You can compile and run this file with cargo by typing
/// ```
///   cargo run --bin get_home_dir
/// ```
pub fn main() {
    match env::home_dir() {
        Some(path) => println!("This is how you get your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}