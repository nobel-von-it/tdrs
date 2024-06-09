# Task Manager

A simple command line task manager written in Rust.

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/tdr.git
   ```
2. Change into the directory:
   ```
   cd tdr
   ```
3. Build and run the application:
   ```
   cargo run
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
