use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

extern crate dirs;

fn check_file(global: bool) -> bool {
    let base_path = if global {
        dirs::home_dir().unwrap().to_string_lossy().to_string()
    } else {
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };

    return std::fs::metadata(format!(
        "{base_path}/rudo/store.json",
        base_path = base_path
    ))
    .is_ok();
}

fn read_file(global: bool) -> Result<TodoList> {
    let base_path = if global {
        dirs::home_dir().unwrap().to_string_lossy().to_string()
    } else {
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!(
            "{base_path}/rudo/store.json",
            base_path = base_path
        ))
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
        TodoList { list: vec![] }
    }

    // Check for the existence of a
    // store file, then initialize
    // the list with its items
    pub fn init(&mut self, global: bool) {
        if check_file(global) {
            let todo_list: TodoList = read_file(global).unwrap();
            self.list = todo_list.list;
        }
    }

    // Add an item to the todolist
    pub fn add_item(&mut self, name: Option<String>) {
        let item: TodoItem = TodoItem::create(name.unwrap());
        self.list.push(item);
    }

    // Toggle the completion state of
    // an item in the todoolist
    pub fn mark_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap()
        {
            panic!("Please provide a valid todo item index.")
        }

        self.list[parsed_index - 1].completed = !self.list[parsed_index - 1].completed;
    }

    // Remove an item from the todolist
    pub fn remove_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap()
        {
            panic!("Please provide a valid todo item index.")
        }

        self.list.remove(parsed_index - 1);
    }

    pub fn clear(&mut self) {
        self.list.clear();

        println!("List cleared.");
    }

    // Print an item
    pub fn print_item(&mut self, index: String) {
        let parsed_index: usize = index.parse::<usize>().unwrap();

        if parsed_index == 0 || parsed_index > self.list.len().to_string().parse::<usize>().unwrap()
        {
            panic!("Please provide a valid todo item index.")
        }

        let todo = &self.list[parsed_index - 1];
        let state: &str = if todo.completed { "x" } else { " " };

        println!("{}. [{}] {:#?}", index, state, todo.name);
    }

    // Dump the todolist to the store file
    pub fn save(&self, global: bool) -> std::io::Result<()> {
        let base_path = if global {
            dirs::home_dir().unwrap().to_string_lossy().to_string()
        } else {
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        };

        fs::create_dir_all(format!("{base_path}/rudo/", base_path = base_path))
            .expect("Couldn't create store file.");

        // Open the log file
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!(
                "{base_path}/rudo/store.json",
                base_path = base_path
            ))
            .expect("Couldn't open the store file.");

        // Truncate the store file
        file.set_len(0).unwrap();

        // Dump the serialized JSON todolist
        serde_json::to_writer(&file, &self).unwrap();

        Ok(())
    }

    // Print the elments of the todolist
    pub fn print(&self) {
        for (i, todo) in self.list.iter().enumerate() {
            let state: &str = if todo.completed { "x" } else { " " };

            println!("{}. [{}] {:#?}", i + 1, state, todo.name)
        }

        self.print_completion();
    }

    // Print the completion status
    pub fn print_completion(&self) {
        let items: u32 = self.list.len() as u32;
        let completed_items: u32 = self.list.iter().filter(|todo| todo.completed).count() as u32;
        let completion_status: f32 = (completed_items as f32 / items as f32) * 100f32;

        println!();
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
