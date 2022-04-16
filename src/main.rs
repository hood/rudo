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
    global: bool
}

fn main() {
    // Instantiate the todo list and
    // initialize it
    let mut todo_list: TodoList;
    todo_list = TodoList::create();

    let opt = Opt::from_args();

    todo_list.init(opt.global);

    if opt.add != None {
        todo_list.add_item(opt.add.to_owned());
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
    else if opt.read != None {
        todo_list.print_item(opt.read.unwrap().to_string());
    } else  if opt.list != false {
        todo_list.print();
    }

    // Save the list before exiting
    // (only on list-modifying actions)
    if opt.add != None || opt.mark != None || opt.delete != None {
        todo_list.save(opt.global).unwrap();
    }
}
