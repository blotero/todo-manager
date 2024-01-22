use std::fs;
use std::io;
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
    let mut buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_delete_task_submenu() {
    display_current_tasks();
    let mut buff = String::new();
    println!("Select task id to delete:");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_modify_task_submenu() {
    let mut buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_mark_task_as_done_submenu() {
    let mut buff = String::new();
    println!("Enter task title>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task description>>> ");
    io::stdin().read_line(&mut buff).unwrap();
    println!("Enter task priority>>> ");
    io::stdin().read_line(&mut buff).unwrap();
}

fn launch_main_menu() {
    println!("========== MAIN MENU ==========");
    println!("1. Create new task");
    println!("2. Delete task");
    println!("3. Modify task");
    println!("4. Mark task as done");
    println!("5. Print all tasks\n");
    println!("Your option: => ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut option: u8 = 0;
    match input.trim().parse::<u8>() {
        Ok(n) => option = n,
        Err(e) => println!("Error: {e}"),
    }
    println!("You have chose the option: {option}");
    match option {
        1 => launch_create_task_submenu(),
        2 => launch_delete_task_submenu(),
        3 => launch_modify_task_submenu(),
        4 => launch_mark_task_as_done_submenu(),
        5 => display_current_tasks(),
        _ => println!("Please select a valid option"),
    }
}

fn main() {
    display_welcome_msg();
    display_current_tasks();
    launch_main_menu();
}
