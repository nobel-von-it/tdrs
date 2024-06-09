use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Command line arguments for the task manager.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available commands for the task manager.
#[derive(Subcommand)]
enum Commands {
    /// Add a new task.
    Add {
        /// The text of the task.
        text: String,
    },
    /// Complete a task.
    Complete {
        /// The ID of the task to complete.
        id: usize,
    },
    /// Uncomplete a task.
    Uncomplete {
        /// The ID of the task to uncomplete.
        id: usize,
    },
    /// Get a task.
    Get {
        /// The ID of the task to get.
        id: usize,
    },
    /// Remove a task.
    Remove {
        /// The ID of the task to remove.
        id: usize,
    },
    /// List all tasks.
    List,
    /// Clear all tasks.
    Clear,
}

/// A list of tasks.
#[derive(Serialize, Deserialize)]
struct TaskList {
    /// The tasks in the list.
    tasks: Vec<Task>,
}

impl TaskList {
    /// Create a new task list from the default file.
    fn new() -> TaskList {
        TaskList::load()
    }

    /// Load the task list from the default file.
    fn load() -> TaskList {
        let default_path = get_default_path();
        let file_path = format!("{}/tasks.json", default_path);
        let _ = std::fs::create_dir_all(default_path.clone());
        let file = match std::fs::File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                let _ = std::fs::File::create(&file_path).unwrap();
                std::fs::File::open(&file_path).unwrap()
            }
        };
        let tasks: TaskList = serde_json::from_reader(file).unwrap_or(TaskList { tasks: vec![] });
        tasks
    }

    /// Save the task list to the default file.
    fn save(&self) {
        let default_path = get_default_path();
        let _ = std::fs::create_dir_all(default_path.clone());
        let mut file = std::fs::File::create(format!("{}/tasks.json", default_path)).unwrap();
        serde_json::to_writer(&mut file, &self)
            .map_err(|e| eprint!("{}", e))
            .unwrap();
    }
}

/// Get the default path for the task list file.
fn get_default_path() -> String {
    match std::env::consts::OS {
        "windows" => format!("{}\\AppData\\Local\\tdr\\", whoami::username()),
        "linux" => format!("/home/{}/.tdr/", whoami::username()),
        _ => format!("/home/{}/.tdr/", whoami::username()),
    }
}

/// A task.
#[derive(Serialize, Deserialize)]
struct Task {
    /// The ID of the task.
    id: usize,
    /// The text of the task.
    text: String,
    /// Whether the task is completed.
    completed: bool,
}

impl fmt::Display for Task {
    /// Format the task for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.completed {
            write!(f, "[x] {}", self.text)
        } else {
            write!(f, "[ ] {}", self.text)
        }
    }
}

impl Task {
    /// Create a new task.
    fn new(id: usize, text: String) -> Task {
        Task {
            id,
            text,
            completed: false,
        }
    }

    /// Complete the task.
    fn complete(&mut self) {
        self.completed = true;
    }

    /// Uncomplete the task.
    fn uncomplete(&mut self) {
        self.completed = false;
    }
}

fn main() {
    let args = Args::parse();
    let mut tasks = TaskList::new();
    match args.command {
        Some(Commands::Add { text }) => {
            tasks
                .tasks
                .push(Task::new(tasks.tasks.len() + 1, text.clone()));
            tasks.save();
            println!("Added task {} with text {}", tasks.tasks.len(), text);
        }
        Some(Commands::Complete { id }) => {
            if let Some(task) = tasks.tasks.get_mut(id - 1) {
                task.complete();
                tasks.save();
                println!("Completed task {}", id);
            }
        }
        Some(Commands::Uncomplete { id }) => {
            if let Some(task) = tasks.tasks.get_mut(id - 1) {
                task.uncomplete();
                tasks.save();
                println!("Uncompleted task {}", id);
            }
        }
        Some(Commands::Remove { id }) => {
            if tasks.tasks.get(id - 1).is_some() {
                tasks.tasks.remove(id - 1);
                tasks.save();
                println!("Removed task {}", id);
            }
        }
        Some(Commands::Get { id }) => {
            if let Some(task) = tasks.tasks.get(id - 1) {
                println!("{}", task);
            }
        }
        Some(Commands::List) => {
            for (i, task) in tasks.tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
            }
        }
        Some(Commands::Clear) => {
            tasks.tasks.clear();
            tasks.save();
            println!("Cleared all tasks");
        }
        None => {
            println!("No command specified");
        }
    }
}
