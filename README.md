# TASC - Task Administration and Scheduling CLI

TASC is a command-line application for task administration and scheduling. It allows you to easily create, manage, organize, and complete tasks using simple CLI commands.

## Features

- **Add tasks**: Create new tasks or sub-tasks with straightforward syntax.
- **Remove tasks**: Delete individual tasks or recursively remove tasks along with their sub-tasks.
- **Mark tasks as completed**: Mark a task as completed.
- **Task management**: Clear all tasks or remove only completed tasks from the list.

## Installation

TASC requires [Rust](https://www.rust-lang.org/) to build the binary. To install and run TASC, follow these steps:

1. Clone the repository to your local machine:
    ```bash
    git clone https://github.com/yourusername/tasc.git
    cd tasc
    ```

2. Build the release version of the application:
    ```bash
    cargo build --release
    ```

3. Move the generated binary to your system's `bin` directory:
    ```bash
    mv target/release/tasc /usr/local/bin/tasc
    ```

4. Ensure that your system has the `$HOME` environment variable set, as it is required for TASC to function properly.

## Usage

Here are the available commands for TASC:

```bash
task <command> [options]
```

### Commands

- `task add <text>`
  Add a new task with the provided text.

- `task add -s <number> <text>`
  Add a new sub-task under the task specified by `<number>`.

- `task remove <number>`
  Remove the task specified by `<number>`.

- `task remove -r <number>`
  Recursively remove the task specified by `<number>` and all its sub-tasks.

- `task mark <number>`
  Mark the task specified by `<number>` as completed.

- `task clear`
  Remove all tasks.

- `task purge`
  Remove all completed tasks from the list.

### Options

- `-h, --help`
  Show the help message and exit.

- `-v, --version`
  Show the current version of TASC.

### Examples

- Add a new task:
    ```
    task add "Buy groceries"
    ```

- Add a sub-task under task 1:
    ```
    task add -s 1 "Buy vegetables"
    ```

- Remove task 3:
    ```
    task remove 3
    ```

- Recursively remove task 3 and all its sub-tasks:
    ```
    task remove -r 3
    ```

- Mark task 2 as completed:
    ```
    task mark 2
    ```

- Clear all tasks:
    ```
    task clear
    ```

- Purge all completed tasks:
    ```
    task purge
    ```

