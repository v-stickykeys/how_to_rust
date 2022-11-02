use std::collections::HashMap;
use std::ops::{Index, Range};

struct Program {
    commands: HashMap<String, Command>
}

impl Program {
    fn add_command(&self, next_command: Command) -> Program {
        let mut commands: HashMap<String, Command> = HashMap::new();
        commands.insert(next_command.name.clone(), next_command);
        for (name, command) in self.commands.iter() {
            // let current_command = Command {
            //     version:
            // }
            commands.insert(name.clone(), command.clone());
        }
        return Program { commands };
    }

    fn get_command(&self, name: String) -> Option<&Command> {
        if !self.commands.contains_key(name.as_str()) {
            println!("Command not found");
            return None;
        }
        return self.commands.get(name.as_str());
    }
}

#[derive(Clone)]
struct Command {
    version: String,
    name: String,
    description: String,
    arguments: Vec<Argument>,
    options: Vec<CommandOption>
}

#[derive(Clone)]
struct Argument {
    name: String,
    description: String,
    default: String
}

#[derive(Clone)]
struct CommandOption {
    flags: String,
    description: String,
    default: String,
    argument: Argument
}

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
    fn program_command_is_found() {
        let command = Command {
            version: String::from("0"),
            name: String::from("first"),
            description: String::from("first command"),
            arguments: Vec::new(),
            options: Vec::new()
        };
        let program = Program {
            commands: HashMap::new()
        };
        let next_program = program.add_command(command);
        let found_command = next_program.get_command(String::from("first"));
        match found_command {
            Some(cmd) => assert_eq!(cmd.name, String::from("first")),
            None => panic!("Failed to get the command")
        }
    }

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