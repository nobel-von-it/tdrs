# Task Manager

A simple command line task manager written in Rust.

## Installation

### Git Clone

1. Clone the repository:
   ```
   git clone https://github.com/nobel-von-it/tdr.git
   ```
2. Change into the directory:
   ```
   cd tdr
   ```
3. Build and run the application:
   ```
   cargo run
   ```

### Cargo install

1. Install the `tdr` package:
   ```
   cargo install tdr
   ```

## Usage

### Commands

- `add <text>`: Add a new task with the given text.
- `complete <id>`: Mark the task with the given ID as completed.
- `uncomplete <id>`: Mark the task with the given ID as uncompleted.
- `remove <id>`: Remove the task with the given ID.
- `get <id>`: Display the task with the given ID.
- `list`: Display all tasks.
- `clear`: Clear all tasks.

### Subcommands

- `subtask <id> add <text>`: Add a new subtask to the task with the given ID.
- `subtask <id> complete <id>`: Mark the subtask with the given ID as completed.
- `subtask <id> uncomplete <id>`: Mark the subtask with the given ID as uncompleted.
- `subtask <id> remove <id>`: Remove the subtask with the given ID.
- `subtask <id> get <id>`: Display the subtask with the given ID.
- `subtask <id> list`: Display all subtasks of the task with the given ID.
- `subtask <id> clear`: Clear all subtasks of the task with the given ID.

### Examples

- Add a new task:
  ```
  tdr add "Buy milk"
  ```
- Complete a task:
  ```
  tdr complete 1
  ```
- List all tasks:
  ```
  tdr list
  ```
- Remove a task:
  ```
  tdr remove 1
  ```
- Display a task:
  ```
  tdr get 1
  ```
- Add a subtask to a task:
  ```
  tdr subtask 1 add "Make tea"
  ```
- Complete a subtask:
  ```
  tdr subtask 1 complete 1
  ```
- List all subtasks of a task:
  ```
  tdr subtask 1 list
  ```

## Features

- Tasks are stored in a JSON file in the default directory for the operating system.
- Tasks can be added, completed, uncompleted, removed, and displayed.
- The task list can be cleared.

## Best Practices

### Adding TDR List to Terminal Startup

To enhance the functionality of your task manager, it is recommended to add the `tdr list` command to run automatically every time you start your terminal. This will ensure that you have a quick overview of your tasks without needing to manually run the command each time.

Here's how you can do it:

1. **For Windows Users:**

   - Open the Start Menu and type `regedit`, then press Enter.
   - Navigate to `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run`.
   - Right-click on the right panel and select New > String Value.
   - Name the new value (e.g., `TDR List`) and set its value to the path of your terminal executable followed by the command `tdr list` (e.g., `"C:\Program Files\Git\bin\git-bash.exe" tdr list`).
   - Restart your terminal to see the effect.

2. **For Linux and macOS Users:**
   - Edit your shell configuration file (e.g., `~/.bashrc` or `~/.zshrc`).
   - Add the following line at the end of the file: `tdr list`.
   - Save the file and restart your terminal to see the effect.

By following these steps, you will have the `tdr list` command run automatically every time you start your terminal, providing you with a convenient way to manage your tasks.

## Contributing

Contributions are welcome. Please open an issue or create a pull request if you have any suggestions or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
