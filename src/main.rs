use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Command line arguments for the task manager.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute.
    #[command(subcommand)]
    command: Option<TaskCommands>,
}

/// Available commands for the task manager.
#[derive(Subcommand)]
enum TaskCommands {
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
    /// Add a subtask to a task.
    Subtask {
        /// The ID of the task to add the subtask to.
        id: usize,
        #[command(subcommand)]
        command: Option<SubtaskCommands>,
    },
}

#[derive(Subcommand)]
enum SubtaskCommands {
    /// Add a new subtask.
    Add {
        /// The text of the subtask.
        text: String,
    },
    /// Complete a subtask.
    Complete {
        /// The ID of the subtask to complete.
        id: usize,
    },
    /// Uncomplete a subtask.
    Uncomplete {
        /// The ID of the subtask to uncomplete.
        id: usize,
    },
    /// Get a subtask.
    Get {
        /// The ID of the subtask to get.
        id: usize,
    },
    /// Remove a subtask.
    Remove {
        /// The ID of the subtask to remove.
        id: usize,
    },
    /// List all subtasks.
    List,
    /// Clear all subtasks.
    Clear,
}

/// A list of tasks.
#[derive(Serialize, Deserialize)]
struct TaskList {
    /// The tasks in the list.
    tasks: Vec<TaskJson>,
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
struct TaskJson {
    /// The ID of the task.
    id: usize,
    /// The text of the task.
    text: String,
    /// Whether the task is completed.
    completed: bool,
    /// The subtasks of the task.
    subtasks: Vec<TaskJson>,
}

impl fmt::Display for TaskJson {
    /// Format the task for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.completed {
            write!(f, "[x] {}", self.text)
        } else {
            write!(f, "[ ] {}", self.text)
        }
    }
}

impl TaskJson {
    /// Create a new task.
    fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
            subtasks: vec![],
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
        Some(TaskCommands::Add { text }) => {
            tasks
                .tasks
                .push(TaskJson::new(tasks.tasks.len() + 1, text.clone()));
            println!("Added task {} with text {}", tasks.tasks.len(), text);
        }
        Some(TaskCommands::Complete { id }) => {
            if let Some(task) = tasks.tasks.get_mut(id - 1) {
                task.complete();
                println!("Completed task {}", id);
            }
        }
        Some(TaskCommands::Uncomplete { id }) => {
            if let Some(task) = tasks.tasks.get_mut(id - 1) {
                task.uncomplete();
                println!("Uncompleted task {}", id);
            }
        }
        Some(TaskCommands::Remove { id }) => {
            if tasks.tasks.get(id - 1).is_some() {
                tasks.tasks.remove(id - 1);
                println!("Removed task {}", id);
            }
        }
        Some(TaskCommands::Get { id }) => {
            if let Some(task) = tasks.tasks.get(id - 1) {
                println!("{}", task);
            }
        }
        Some(TaskCommands::List) => {
            for (i, task) in tasks.tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
                for (i, subtask) in task.subtasks.iter().enumerate() {
                    println!("  {}: {}", i + 1, subtask);
                }
            }
        }
        Some(TaskCommands::Clear) => {
            tasks.tasks.clear();
            println!("Cleared all tasks");
        }
        Some(TaskCommands::Subtask { id, command }) => {
            if let Some(task) = tasks.tasks.get_mut(id - 1) {
                match command {
                    Some(SubtaskCommands::Add { text }) => {
                        task.subtasks
                            .push(TaskJson::new(task.subtasks.len() + 1, text.clone()));
                        println!("Added subtask {} with text {}", task.subtasks.len(), text);
                    }
                    Some(SubtaskCommands::Complete { id }) => {
                        if let Some(subtask) = task.subtasks.get_mut(id - 1) {
                            subtask.complete();
                            println!("Completed subtask {}", id);
                        }
                    }
                    Some(SubtaskCommands::Uncomplete { id }) => {
                        if let Some(subtask) = task.subtasks.get_mut(id - 1) {
                            subtask.uncomplete();
                            println!("Uncompleted subtask {}", id);
                        }
                    }
                    Some(SubtaskCommands::Remove { id }) => {
                        if task.subtasks.get(id - 1).is_some() {
                            task.subtasks.remove(id - 1);
                            println!("Removed subtask {}", id);
                        }
                    }
                    Some(SubtaskCommands::Get { id }) => {
                        if let Some(subtask) = task.subtasks.get(id - 1) {
                            println!("{}", subtask);
                        }
                    }
                    Some(SubtaskCommands::List) => {
                        task.subtasks.iter().enumerate().for_each(|(i, s)| {
                            println!("{}: {}", i + 1, s);
                        })
                    }
                    Some(SubtaskCommands::Clear) => {
                        task.subtasks.clear();
                        println!("Cleared all subtasks");
                    }
                    None => {
                        println!("No subtask command specified");
                    }
                }
            }
        }
        None => {
            println!("No command specified");
        }
    }
    tasks.save();
}
