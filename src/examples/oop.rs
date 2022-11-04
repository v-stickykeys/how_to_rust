use std::collections::HashMap;
use std::ops::{Index, Range, Add};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Program {
    name: String,
    version: String,
    description: String,
    parent: Option<Link>,
    children: HashMap<String, Link>,
    arguments: Vec<Argument>,
    options: Vec<ProgramOption>
}
type Link = Rc<RefCell<Program>>;

impl Program {
    fn command(mut self, mut program: Program) -> (Program, Program) {
        let mut child_link = Rc::new(RefCell::new(program.clone()));
        self.children.insert(program.name.clone(), child_link);
        let parent_link = Rc::new(RefCell::new(self.clone()));
        program.parent = Some(parent_link);
        return (self, program);
    }

    fn get_child(&self, name: String) -> Option<&Link> {
        if !self.children.contains_key(name.as_str()) {
            println!("Command not found");
            return None;
        }
        return self.children.get(name.as_str());
    }
}

#[derive(Clone)]
struct Argument {
    name: String,
    description: String,
    default: String
}

#[derive(Clone)]
struct ProgramOption {
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

pub fn parse() {
}

pub fn run() {
    println!("This is how you mimic a `class` in Rustlang!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Ref;

    #[test]
    fn program_command_is_found() {
        let mut parent = Program {
            name: String::from("root"),
            version: String::from("0"),
            description: String::from("root program"),
            parent: None,
            children: HashMap::new(),
            arguments: Vec::new(),
            options: Vec::new()
        };
        let mut command = Program {
            name: String::from("command"),
            version: String::from("0"),
            description: String::from("command program"),
            parent: None,
            children: HashMap::new(),
            arguments: Vec::new(),
            options: Vec::new()
        };
        let (parent, child) = parent.command(command);
        let valid_command = parent.get_child(String::from("command"));
        match valid_command {
            Some(cmd) => assert_eq!(*Ref::map(cmd.borrow(), |n| &n.name), String::from("command")),
            None => panic!("Failed to get the command")
        }
        let invalid_command = parent.get_child(String::from("leaf"));
        match invalid_command {
            Some(cmd) => panic!("Found command that should be invalid"),
            None => return
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