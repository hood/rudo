mod lib;
use lib::{TodoList};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rudo")]
struct Opt {
    #[structopt(short, long)]
    add: Option<String>,

    #[structopt(short, long)]
    mark: Option<i32>,

    #[structopt(short, long)]
    get: Option<i32>,

    #[structopt(short, long)]
    delete: Option<i32>
}

fn main() {
    // Instantiate the todo list and
    // initialize it
    let mut todo_list: TodoList;
    todo_list = TodoList::create();
    todo_list.init();

    let opt = Opt::from_args();

    if opt.add != None {
        todo_list.add_item(opt.add.unwrap());
        todo_list.print();
    }
    else if opt.mark != None {
        todo_list.mark_item(opt.mark.unwrap().to_string());
        todo_list.print();
    }
    else if opt.delete != None {
        todo_list.remove_item(opt.delete.unwrap().to_string());
        todo_list.print();
    }
    else if opt.get != None {
        todo_list.print();
    }

    // Save the list before exiting
    todo_list.save().unwrap();
}
