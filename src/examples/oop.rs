use std::collections::HashMap;
use std::ops::{Index, Range, Add};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Program {
    name: String,
    version: Option<String>,
    description: Option<String>,
    parent: Option<Link>,
    children: HashMap<String, Link>,
    arguments: OptionalProgramArguments,
    options: OptionalProgramOptions,
    action: fn(OptionalProgramArguments, OptionalProgramOptions)
}

type Link = Rc<RefCell<Program>>;
type OptionalProgramArguments = Option<Vec<ProgramArgument>>;
type OptionalProgramOptions = Option<Vec<ProgramOption>>;

impl Program {
    fn new(name: String) -> Self {
        Self {
            name,
            version: None,
            description: None,
            parent: None,
            children: HashMap::new(),
            arguments: None,
            options: None,
            action: |arguments: OptionalProgramArguments, options: OptionalProgramOptions| { println!("Command not implemented"); }
        }
    }

    fn action(mut self, func: fn(OptionalProgramArguments, OptionalProgramOptions)) {
        self.action = func;
    }

    fn command(mut self, name: String) -> (Program, Program) {
        let mut child = Program::new(name);
        let mut child_link = Rc::new(RefCell::new(child.clone()));
        self.children.insert(child.name.clone(), child_link);
        let parent_link = Rc::new(RefCell::new(self.clone()));
        child.parent = Some(parent_link);
        return (self, child);
    }

    fn description(mut self, description: String) -> Program {
        self.description = Some(description);
        return self;
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
struct ProgramArgument {
    name: String,
    description: String,
    default: String
}

#[derive(Clone)]
struct ProgramOption {
    flags: String,
    description: String,
    default: String,
    argument: ProgramArgument
}

impl ProgramArgument {
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
    fn program_action_is_set() {
        let mut program = Program::new(String::from("root"));
        fn help(arguments: OptionalProgramArguments, options: OptionalProgramOptions) {
            match arguments {
                Some(x) => println!("{:?}", x.len()),
                _ => ()
            }
            match options {
                Some(y) => println!("{:?}", y.len()),
                _ => ()
            }
        }
        program.action(help)
    }

    #[test]
    fn program_description_is_set() {
        let mut program = Program::new(String::from("root"));
        program = program.description(String::from("root command"));
        match program.description.clone() {
            Some(description) => assert_eq!(description, String::from("root command")),
            None => panic!("Failed to get description")
        }
    }

    #[test]
    fn program_command_is_found() {
        let mut parent = Program::new(String::from("root"));
        let (parent, child) = parent.command(String::from("command"));
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
        let arg = ProgramArgument {
            name: String::from("<my_required_arg>"),
            default: String::from("run"),
            description: String::from("my required argument"),
        };
        assert!(arg.is_required())
    }
}