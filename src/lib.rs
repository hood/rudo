use serde::{Deserialize, Serialize};
use serde_json::Result;
extern crate dirs;

fn check_file() -> bool {
    return std::fs::metadata(format!("{homedir}/rudo/store.json", homedir = dirs::home_dir().unwrap().to_string_lossy())).is_ok();
}

fn read_file() -> Result<TodoList> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!("{homedir}/rudo/store.json", homedir = dirs::home_dir().unwrap().to_string_lossy()))
        .unwrap();

    let result: TodoList = serde_json::from_reader(&file)?;

    Ok(result)
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    // Return an empty todolist
    pub fn create() -> TodoList {
        return TodoList { list: vec![] };
    }

    // Check for the existence of a
    // store file, then initialize
    // the list with its items
    pub fn init(&mut self) {
        if check_file() {
            let todo_list: TodoList = read_file().unwrap();
            self.list = todo_list.list;
        }
    }

    // Add an item to the todolist
    pub fn add_item(&mut self, name: String) {
        let item: TodoItem = TodoItem::create(name);
        self.list.push(item);
    }

    // Toggle the completion state of
    // an item in the todoolist
    pub fn mark_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap() {
            panic!("Please provide a valid todo item index.")
        }

        self.list[parsed_index - 1].completed = !self.list[parsed_index - 1].completed;
    }

    // Remove an item from the todolist
    pub fn remove_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap() {
            panic!("Please provide a valid todo item index.")
        }

        &self.list.remove(parsed_index - 1);
    }

    // Print an item
    pub fn print_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap() {
            panic!("Please provide a valid todo item index.")
        }

        let todo = &self.list[parsed_index - 1];

        let state: String;

        if todo.completed == true {
            state = "x".to_string();
        } else {
            state = " ".to_string();
        };

        println!("{}. [{}] {:#?}", index, state, todo.name); 
    }

    // Dump the todolist to the store file
    pub fn save(&self) -> std::io::Result<()> {
        // Open the log file
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{homedir}/rudo/store.json", homedir = dirs::home_dir().unwrap().to_string_lossy()))
            .unwrap();

        // Truncate the store file
        file.set_len(0).unwrap();

        // Dump the serialized JSON todolist
        serde_json::to_writer(&file, &self).unwrap();

        Ok(())
    }

    // Print the elments of the todolist
    pub fn print(&self) {
        if self.list.is_empty() {
            ()
        }

        for (i, todo) in self.list.iter().enumerate() {
            let state: String;
            if todo.completed == true {
                state = "x".to_string();
            } else {
                state = " ".to_string();
            };

            println!("{}. [{}] - {:#?}", i + 1, state, todo.name)
        }

        self.print_completion();
    }

    // Print the completion status
    pub fn print_completion(&self) {
        let items: u32 = self.list.len() as u32;
        let mut completed_items: u32 = 0;

        for (_, todo) in self.list.iter().enumerate() {
            if todo.completed == true {
                completed_items += 1;
            }
        }

        let completion_status: f32 = (completed_items as f32 / items as f32) * 100f32;

        println!("");
        println!("{} of {} tasks completed", completed_items, items);

        println!("Completion status: {}%", completion_status as u32);
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub name: String,
    pub completed: bool,
}

impl TodoItem {
    fn create(name: String) -> TodoItem {
        return TodoItem {
            name: name.trim().to_string(),
            completed: false,
        };
    }
}
