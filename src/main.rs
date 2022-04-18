mod lib;
use lib::TodoList;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rudo")]
struct Opt {
    #[structopt(short, long)]
    add: Option<String>,

    #[structopt(short, long)]
    mark: Option<i32>,

    #[structopt(short, long)]
    read: Option<i32>,

    #[structopt(short, long)]
    delete: Option<i32>,

    #[structopt(short, long)]
    list: bool,

    #[structopt(short, long)]
    global: bool,
}

fn main() {
    let mut todo_list: TodoList = TodoList::create();

    let option = Opt::from_args();

    todo_list.init(option.global);

    if option.add != None {
        todo_list.add_item(option.add.to_owned());
        todo_list.print();
    } else if let Some(value) = option.mark {
        todo_list.mark_item(value.to_string());
        todo_list.print();
    } else if let Some(value) = option.delete {
        todo_list.remove_item(value.to_string());
        todo_list.print();
    } else if let Some(value) = option.read {
        todo_list.print_item(value.to_string());
    } else if option.list {
        todo_list.print();
    }

    // Save the list before exiting (only on list-modifying actions).
    if option.add != None || option.mark != None || option.delete != None {
        todo_list.save(option.global).unwrap();
    }
}
