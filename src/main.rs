use std::env;

fn main() {
    // Get the arguments passed to the CLI
    let args: Vec<String> = env::args().collect();

    // Check the first argument to determine the command
    if args.len() < 2 {
        println!("Usage: tasks_cli <command> [arguments]");
        println!("Commands:");
        println!("  add <task>      Add a new task");
        println!("  list            List all tasks");
        println!("  complete <id>   Mark a task as complete");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add_task(args),
        "list" => list_tasks(),
        "complete" => complete_task(args),
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn add_task(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: tasks_cli add <task>");
        return;
    }

    let task = args[2..].join(" ");
    println!("Task added: {}", task);
}

fn list_tasks() {
    println!("Here are your tasks:");
    // Later, we’ll fetch and display tasks here
}

fn complete_task(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: tasks_cli complete <id>");
        return;
    }

    let task_id = &args[2];
    println!("Task {} marked as complete", task_id);
}
