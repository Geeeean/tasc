# TASC - Task Administration and Scheduling CLI

TASC is a command-line application for task administration and scheduling. It allows you to easily create, manage, organize, and complete tasks using simple CLI commands.

## TODO

- Add error handling for out-of-bounds task numbers in commands that require a task number.
- Add error handling for executing a normal command instead of a recursive command in situations where it is required.
- Implement functionality so that when sub-tasks of a task are marked as completed, the parent task is also marked as completed automatically.

## Contributing

I welcome contributions to TASC! If you'd like to contribute, please feel free to submit a pull request. Whether you want to report a bug, suggest an enhancement, or improve the documentation, your input is valuable.

## Features

- **Add tasks**: Create new tasks or sub-tasks with straightforward syntax.
- **Remove tasks**: Delete individual tasks or recursively remove tasks along with their sub-tasks.
- **Mark tasks as completed**: Mark a task as completed.
- **Task management**: Clear all tasks or remove only completed tasks from the list.

## Installation

TASC requires [Rust](https://www.rust-lang.org/) to build the binary. To install and run TASC, follow these steps:

1. Clone the repository to your local machine:
    ```bash
    git clone https://github.com/Geeeean/tasc.git
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

- `task mark -r <number>`
  Recursively mark the task specified by `<number>` and all its sub-tasks.

- `task unmark <number>`
  Unmark the task specified by `<number>` as completed.

- `task unmark -r <number>`
  Recursively unmark the task specified by `<number>` and all its sub-tasks.

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

![Dimostrazione della CLI](https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExdHVvaG1kZHNmZmk4YTE1MGh2a3RtbnoxeW0zdmVsaXh6OW0zbGVzNiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/lLjs7ttfk5oMTor54T/giphy.gif)

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

