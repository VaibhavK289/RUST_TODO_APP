use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const FILE_PATH: &str = ".todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: 'add' command requires a task description.");
                return;
            }
            let task = args[2..].join(" ");
            add_task(&task);
        }
        "list" => {
            list_tasks();
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Error: 'done' command requires a task number.");
                return;
            }
            if let Ok(index) = args[2].parse::<usize>() {
                mark_done(index);
            } else {
                eprintln!("Error: Invalid task number.");
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: 'remove' command requires a task number.");
                return;
            }
            if let Ok(index) = args[2].parse::<usize>() {
                remove_task(index);
            } else {
                eprintln!("Error: Invalid task number.");
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: todo-cli <command> [arguments]");
    println!("Commands:");
    println!("  add <task>   - Add a new task");
    println!("  list         - List all tasks");
    println!("  done <num>   - Mark a task as done (1-based index)");
    println!("  remove <num> - Remove a task (1-based index)");
}

fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
        .expect("Failed to open todo file");
    
    writeln!(file, "[ ] {}", task).expect("Failed to write to todo file");
    println!("Added task: {}", task);
}

fn read_tasks() -> Vec<String> {
    let file = match fs::File::open(FILE_PATH) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

fn write_tasks(tasks: &[String]) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open todo file");
    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to todo file");
    }
}

fn list_tasks() {
    let tasks = read_tasks();
    if tasks.is_empty() {
        println!("No tasks found. Yay!");
        return;
    }
    println!("Todo List:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task);
    }
}

fn mark_done(index: usize) {
    let mut tasks = read_tasks();
    if index == 0 || index > tasks.len() {
        eprintln!("Error: Task number out of bounds.");
        return;
    }
    let task = &mut tasks[index - 1];
    if task.starts_with("[ ]") {
        *task = task.replacen("[ ]", "[x]", 1);
        println!("Marked task {} as done.", index);
        write_tasks(&tasks);
    } else {
        println!("Task {} is already done.", index);
    }
}

fn remove_task(index: usize) {
    let mut tasks = read_tasks();
    if index == 0 || index > tasks.len() {
        eprintln!("Error: Task number out of bounds.");
        return;
    }
    println!("Removed task: {}", tasks[index - 1]);
    tasks.remove(index - 1);
    write_tasks(&tasks);
}