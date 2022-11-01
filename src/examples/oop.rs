use std::ops::{Index, Range};

// struct Command {
//     version: String,
//     name: String,
//     description: String,
//     arguments: Vec<Argument>,
//     options: Vec<Option>
// }

struct Argument {
    name: String,
    description: String,
    default: String
}

// struct Option {
//     flags: String,
//     description: String,
//     default: String,
//     arguments: Vec<Argument>
// }

impl Argument {
    fn is_required(&self) -> bool {
        if self.name.len() > 0 {
            return self.name.index(Range{start: 0, end: 1}) == "<";
        }
        return false;
    }
    fn print(&self) {
        println!("{}, {}, {}", self.name, self.description, self.default);
    }
}

pub fn run() {
    println!("This is how you mimic a `class` in Rustlang!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inequality_symbol_means_required() {
        let arg = Argument {
            name: String::from("<my_required_arg>"),
            default: String::from("run"),
            description: String::from("my required argument"),
        };
        assert!(arg.is_required())
    }
}