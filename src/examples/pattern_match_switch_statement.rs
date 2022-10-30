use crate::examples::{get_home_dir, import_function, console_log};

/// This use of the match method is called "pattern matching" in Rustlang. It is similar to a
/// switch statement.
pub fn run(command: &String) {
    match command.as_str() {
        "get_home_dir" => get_home_dir::run(),
        "import_function" => import_function::run(),
        "console_log" => console_log::main(),
        _ => println!("Invalid command")
    }
}