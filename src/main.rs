mod lib;
use lib::{Command, TodoList};

fn main() {
    // Collect arguments
    let args: Vec<String> = std::env::args().collect();

    // Instantiate the todo list and
    // initialize it
    let mut todo_list: TodoList;
    todo_list = TodoList::create();
    todo_list.init();

    // First argument passed is the command
    let term_command = args[1].clone();

    // Parse the command string
    let command = match term_command.as_str() {
        "get" => Command::GET,
        "add" => Command::ADD,
        "mark" => Command::MARK,
        "delete" => Command::DELETE,
        _ => panic!("Must provide a valid command. Valid commands are \"get\", \"add\", \"mark\", \"delete\"."),
    };

    // Define the argument and initialize it
    // to a dummy value
    let mut subject: String = "NO_SUBJECT".to_string();

    // Second command passed is the argument
    if term_command != "get" {
        subject = args[2].clone();
    }

    // Execute the passed command
    match command {
        Command::GET => todo_list.print(),
        Command::ADD => {
            todo_list.add_item(subject.to_string());
            todo_list.print();
        }
        Command::MARK => {
            todo_list.mark_item(subject.to_string());
            todo_list.print();
        }
        Command::DELETE => {
            todo_list.remove_item(subject.to_string());
            todo_list.print();
        }
    }

    // Save the list before exiting
    todo_list.save().unwrap();
}
