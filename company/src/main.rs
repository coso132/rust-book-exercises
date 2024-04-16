use core::panic;
use std::{collections::HashMap, io::stdin};
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Department {
    Security,
    Engineering,
    Sales,
}
#[allow(dead_code)]
impl Department {
    fn from_string(s: &str) -> Option<Department> {
        Some(match s.to_lowercase().trim() {
            "se" | "sec" | "security" => Department::Security,
            "e" | "eng" | "engineering" => Department::Engineering,
            "sa" | "sal" | "sales" => Department::Sales,
            _ => return None,
        })
    }
    fn list_of_all_departments() -> Vec<Department> {
        vec![
            Department::Security,
            Department::Engineering,
            Department::Sales,
        ]
    }
}
#[derive(Debug)]
enum Command {
    Empty,
    Quit,
    AddTo(String, Department),
    Remove(String),
    ViewList,
}
impl Command {
    fn parse_command(s: String) -> Command {
        let mut iteratorwords = s.trim().split(' ').into_iter();
        match iteratorwords.next() {
            None => Command::Empty,
            Some(word) => {
                match word {
                    "Quit" | "q" | "quit" | "Q" => return Command::Quit,
                    "View" | "v" | "view" | "V" => return Command::ViewList,
                    "Remove" | "r" | "rm" | "remove" => {
                        return Command::Remove(match iteratorwords.next() {
                            Some(s) => s.to_owned(),
                            _ => return Command::Empty,
                        })
                    }
                    "Add" | "add" => {
                        return Command::AddTo(
                            match iteratorwords.next() {
                                None => return Command::Empty,
                                Some(s) => s.to_owned(),
                            },
                            match iteratorwords.next() {
                                Some("To") | Some("to") => {
                                    match iteratorwords.next() {
                                        Some(s) => {
                                            match Department::from_string(s) {
                                                Some(d) => d, //If a valid department name has been entered,
                                                None => return Command::Empty,
                                            }
                                        }
                                        _ => return Command::Empty,
                                    }
                                }
                                _ => return Command::Empty,
                            },
                        );
                    }
                    _ => return Command::Empty,
                }
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Company {
    name: String,
    employees: HashMap<String, Department>,
}
impl Company {
    fn new() -> Self {
        Self {
            name: "Fornite".to_owned(),
            employees: (HashMap::new()),
        }
    }
    fn execute_command(&mut self, c: Command) {
        match c {
            Command::Quit => panic!(),
            Command::AddTo(name, dep) => _ = self.employees.insert(name, dep),
            Command::Remove(name) => _ = self.employees.remove::<String>(&name),
            Command::ViewList => {
                let departments = Department::list_of_all_departments();
                let sorted_employess: Vec<(String, Department)> =
                    self.employees.clone().into_iter().collect();
                for d in departments {
                    println!("Employees in the {:#?} department:", &d);
                    for (key, value) in &sorted_employess {
                        if d == *value {
                            println!("{key}");
                        }
                    }
                }
            }
            Command::Empty => (),
        }
    }
}
fn main() {
    println!("Ciao!");
    let mut test_company = Company::new();
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("minchia");
        test_company.execute_command(Command::parse_command(s));
    }
}
