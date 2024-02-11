mod record;

use record::DataRecord;
use std::fs;
use std::io::{self, Error, ErrorKind};

use String;

fn print_data_from_raw(raw: String) {
    // Todo: fancier print
    let lines = raw.split("\n");
    for (i, line) in lines.enumerate() {
        if line.len() > 1 {
            let task_index = i + 1;
            println!("{task_index}. {line}")
        }
    }
}

fn read_data_file(path: String) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    print_data_from_raw(contents);
}

fn display_welcome_msg() {
    println!("Welcome to your Rust task manager!");
}

fn get_data_path() -> String {
    // Todo:
    // - Seek path data/data.txt by default
    // - Handle error if file does not exists
    // - Read the config file as second priority for seeking
    //   data path
    return String::from("data/data.txt");
}

fn display_current_tasks() {
    println!("These are your current tasks:");
    let path = get_data_path();
    read_data_file(path);
}

fn launch_create_task_submenu() {
    println!("Creating new task!");
    let mut name_buff = String::new();
    let mut desc_buff = String::new();
    let mut priority_buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut name_buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut desc_buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut priority_buff).unwrap();
    let mut priority: u8 = 0;

    let mut succ = false;
    while !succ {
        match priority_buff.trim().parse::<u8>() {
            Ok(n) => {
                priority = n;
                succ = true
            }
            Err(e) => {
                println!("Error parsing priority: {e}");
                println!("Please type a valid priority>>> ");
                priority_buff.clear();
                io::stdin().read_line(&mut priority_buff).unwrap();
            }
        }
    }
    let record = DataRecord {
        name: name_buff.trim(),
        description: &desc_buff.trim(),
        priority,
        is_done: false,
    };
    match record.save() {
        Ok(n) => println!("Successfully saved record!. Wrote {} bytes", n),
        Err(err) => println!("Error saving record :( {}", err),
    }
}

fn launch_delete_task_submenu() {
    println!("Deleting task...");
    display_current_tasks();
    let mut buff = String::new();
    println!("Select task id to delete:");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_modify_task_submenu() {
    println!("Modifying existing task!");
    let mut buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_mark_task_as_done_submenu() {
    println!("Marking task as done!");
    let mut buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut buff).unwrap();
}
struct MenuOption {
    option_text: String,
    option_key: u8,
    callback: fn(),
}

fn populate_menu_options(op_map: &mut Vec<MenuOption>) {
    op_map.push(MenuOption {
        option_text: String::from("Create new task"),
        option_key: 1,
        callback: launch_create_task_submenu,
    });
    op_map.push(MenuOption {
        option_text: String::from("Delete task"),
        option_key: 2,
        callback: launch_delete_task_submenu,
    });
    op_map.push(MenuOption {
        option_text: String::from("Modify task"),
        option_key: 3,
        callback: launch_modify_task_submenu,
    });
    op_map.push(MenuOption {
        option_text: String::from("Mark task as done"),
        option_key: 4,
        callback: launch_mark_task_as_done_submenu,
    });
    op_map.push(MenuOption {
        option_text: String::from("Print all tasks"),
        option_key: 5,
        callback: display_current_tasks,
    });
}

fn run_by_option_num(op_num: u8, op_map: &Vec<MenuOption>) -> Result<u8, Error> {
    for op in op_map.iter() {
        if op_num == op.option_key {
            (op.callback)();
            return Ok(op_num);
        }
    }
    return Err(io::Error::new(
        ErrorKind::NotFound,
        format!("Option: {} not available", op_num),
    ));
}

fn launch_main_menu(op_map: &Vec<MenuOption>) {
    println!("========== MAIN MENU ==========");

    for op in op_map.iter() {
        let key = op.option_key.to_string();
        let text = &op.option_text;
        println!("{key}. {text}");
    }

    println!("Your option: => ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut option: u8 = 0;
    let mut parse_succ = false;
    let mut run_succ = false;
    while !parse_succ || !run_succ {
        match input.trim().parse::<u8>() {
            Ok(n) => {
                option = n;
                parse_succ = true
            }
            Err(e) => {
                println!("Error: {e}");
                println!("Please type option again>>> ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
            }
        }
        match run_by_option_num(option, &op_map) {
            Ok(_) => run_succ = true,
            Err(err) => {
                println!("Error running option: {err}");
                println!("Plese provide a valid option>>> ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
            }
        }
    }
}

fn main() {
    let mut op_map: Vec<MenuOption> = Vec::new();
    populate_menu_options(&mut op_map);
    display_welcome_msg();
    display_current_tasks();
    loop {
        launch_main_menu(&op_map);
    }
}
