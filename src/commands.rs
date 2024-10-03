
use std::{env, io, fs};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn format_line(line: &(String, usize), prev_depth: usize, next_depth: usize) -> Option<String> {
    let mut formatted = String::new();

    let line_depth = line.1;
    let line = &line.0;

    let chunks: Vec<&str> = line.split(' ').collect();

    let marked = match chunks.get(0) {
        Some(&x) => x == "x",
        None => return None,
    };

    if marked {
        formatted.push_str("[x]");
    } else {
        formatted.push_str("[ ]");
    }

    formatted.push(' ');

    if line_depth > 1 {
        formatted.push_str(&"│  ".repeat(line_depth-1)) ;

        if line_depth == next_depth {
            formatted.push_str("├─");
        } else {
            formatted.push_str("└─");
        }
    } else {
        formatted.push_str("├─");
    }

    formatted.push(' ');

    match marked {
        true => formatted.push_str(&format!("\x1B[9m{}\x1B[0m", &chunks[2..].join(" "))),
        false => formatted.push_str(&chunks[2..].join(" ")),
    }

    Some(formatted)
}

pub fn list(file: File) -> Result<(), io::Error> {
    let reader = BufReader::new(file);

    //a bit of shadowing here 🙈
    let lines: io::Result<Vec<String>> = reader.lines().collect();

    let lines = match lines {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let lines: io::Result<Vec<(String, usize)>> = lines.into_iter().map(|line| {
        let chunks: Vec<&str> = line.split(' ').collect();

        let line_depth = match chunks.get(1) {
            Some(&x) => x.parse::<usize>(),
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
        };

        let line_depth = match line_depth {
            Ok(num) => num,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
        };

        Ok((line, line_depth))
    }).collect();

    let lines = match lines {
        Ok(res) => res,
        Err(r) => return Err(r),
    };

    for (i, line) in lines.iter().enumerate() {
        let prev_depth = if i != 0 {
            match lines.get(i-1) {
                Some(x) => x.1,
                None => 0,
            }
        } else {
            0
        };

        let next_depth = match lines.get(i+1) {
            Some(x) => x.1,
            None => 0,
        };

        match format_line(line, prev_depth, next_depth) {
            Some(str) => println!("{:0>2} {}", i+1, str),
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line.")),
        };
    }

    Ok(())
}

pub fn purge(file: File, file_path: &Path, tmp_file_path: &Path) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let marked = match l.chars().next() {
                    Some(x) => x == 'x',
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line.")),
                };

                if !marked {
                    writeln!(tmp_file, "{}", l)?;
                }
            },
            Err(r) => return Err(r),
        }
    }

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

pub fn add(file: File, file_path: &Path, tmp_file_path: &Path,  args: Vec<String>) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    let sub_task = match args.get(2) {
        Some(x) => x == "-s",
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args.")),
    };

    let mut sub_task_parent: usize = 0;
    if sub_task {
        let parent_string = match args.get(3) {
            Some(x) => x,
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args.")),
        };

        sub_task_parent = match parent_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
        }
    }

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                writeln!(tmp_file, "{}", l)?;
                if i+1 == sub_task_parent {
                    let chunks: Vec<&str> = l.split(' ').collect();
                    let line_depth = match chunks.get(1) {
                        Some(&x) => x.parse::<usize>(),
                        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
                    };

                    let line_depth = match line_depth {
                        Ok(num) => num,
                        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
                    };

                    writeln!(tmp_file, "o {} {}", line_depth+1, args[4..].join(" "))?;
                }
            },
            Err(r) => return Err(r),
        }
    }

    if !sub_task {
        writeln!(tmp_file, "o 1 {}", args[2..].join(" "))?;
    }

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

pub fn mark(file: File, file_path: &Path, tmp_file_path: &Path, args: Vec<String>) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    let line_number = match args.get(2) {
        Some(x) => x,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args.")),
    };

    let line_number = match line_number.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
    };

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                if i+1 == line_number {
                    let slice: Vec<&str> = l.split(' ').collect();
                    let remaining = slice[1..].join(" ");
                    writeln!(tmp_file, "x {}", remaining)?;
                } else {
                    writeln!(tmp_file, "{}", l)?;
                }
            },
            Err(r) => return Err(r),
        }
    }

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

pub fn clear(file: File, file_path: &Path, tmp_file_path: &Path) -> Result<(), io::Error> {
    let tmp_file = File::create(tmp_file_path)?;

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

pub fn remove(file: File, file_path: &Path, tmp_file_path: &Path, args: Vec<String>) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    let line_number = match args.get(2) {
        Some(x) => x,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args.")),
    };

    let line_number = match line_number.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format.")),
    };

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                if i+1 != line_number {
                    writeln!(tmp_file, "{}", l)?;
                }
            },
            Err(r) => return Err(r),
        }
    }

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

pub fn help() -> () {
    let help_message = r#"TASC - Task Administration and Scheduling CLI

Usage:
    task <command> [options]

Commands:
    task add <text>                       Add a new task with the provided text.
    task add -s <number> <text>           Add a new sub-task under the task specified by <number>.
    task remove <number>                  Remove the task specified by <number>.
    task mark <number>                    Mark the task specified by <number> as completed.
    task clear                            Remove all tasks.
    task purge                            Remove all completed tasks from the list.

Options:
    -h, --help                            Show this help message and exit.
    -v, --version                         Show the current version of TASC.

Examples:
    task add "Buy groceries"              Adds a new task with the text "Buy groceries".
    task add -s 1 "Buy vegetables"        Adds a sub-task under task 1 with the text "Buy vegetables".
    task remove 3                         Removes the task with ID 3.
    task mark 2                           Marks the task with ID 2 as completed.
    task clear                            Permanently remove all tasks.
    task purge                            Permanently removes all completed tasks from the list."#;

    println!("{}", help_message)
}

pub fn version() -> () {
    let package_name = env!("CARGO_PKG_NAME");
    let package_version = env!("CARGO_PKG_VERSION");

    println!("{} version {}", package_name, package_version);
}

