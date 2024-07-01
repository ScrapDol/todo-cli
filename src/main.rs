mod task;

use dirs;
use std::env;
use std::{fs, path::PathBuf};
use task::Task;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    if args.len() > 2 {
        let description = &args[2];
        match query.as_str() {
            "add" => add_task(description.clone()),
            "list" => list_tasks(),
            "comp" => complite_task(description.clone().parse().expect("error")),
            "delete" => delete_task(description.clone().parse().expect("error")),
            _ => println!("Unknown command"),
        }
    } else {
        match query.as_str() {
            "list" => list_tasks(),
            _ => println!("unknown command"),
        }
    }
}

fn get_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push("exemple.json");
    path
}

fn load_tasks() -> Vec<Task> {
    let path = get_file_path();

    if path.exists() {
        let date = fs::read_to_string(path).expect("error");

        serde_json::from_str(&date).expect("error")
    } else {
        vec![]
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let path = get_file_path();
    let data = serde_json::to_string(tasks).expect("error");
    fs::write(path, data).expect("error");
}

fn add_task(description: String) {
    let mut tasks = load_tasks();

    let id = tasks.len() as u32 + 1;

    let task = Task::new(id, description);
    tasks.push(task);
    save_tasks(&tasks);
    println!("Task added!");
}

fn complite_task(id: u32) {
    let mut tasks = load_tasks();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = !task.completed;
        save_tasks(&tasks);
    } else {
        println!("Task not fount!");
    }
}

fn list_tasks() {
    let tasks = load_tasks();

    for task in tasks {
        println!("{}", task);
    }
}

fn delete_task(id: u32) {
    let mut tasks = load_tasks();

    tasks.remove(usize::try_from(id - 1).expect("error"));
    save_tasks(&tasks)
}

// TODO: Доделать edit_task
// fn edit_task(id: u32, new_description: String) {
//     let mut tasks = load_tasks();

//     if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
//         task.description = new_description;
//         save_tasks(&tasks);
//     } else {
//         println!("Task not fount!");
//     }
// }
