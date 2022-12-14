use crate::examples::{function_as_arg, get_home_dir, import_function, console_log, oop};

/// This use of the match method is called "pattern matching" in Rustlang. It is similar to a
/// switch statement.
pub fn run(command: &String) {
    match command.as_str() {
        "console_log" => console_log::main(),
        "function_as_arg" => function_as_arg::run(),
        "get_home_dir" => get_home_dir::run(),
        "import_function" => import_function::run(),
        "oop" => oop::run(),
        _ => println!("Invalid command")
    }
}