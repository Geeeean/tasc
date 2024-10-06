use crate::error::CommandError;

use std::{env, io, fs};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

type CommandResult = Result<(), CommandError>;

pub struct Command {
    file: File,
    file_path: PathBuf,
    tmp_file_path: PathBuf,
    args: Vec<String>,
}

impl Command {
    pub fn new(file: File, file_path: PathBuf, tmp_file_path: PathBuf, args: Vec<String>) -> Command {
        Command{
            file,
            file_path,
            tmp_file_path,
            args
        }
    }

    pub fn list(&self) -> CommandResult {
        let reader = BufReader::new(&self.file);

        //a bit of shadowing here 🙈
        let lines: io::Result<Vec<String>> = reader.lines().collect();
        let lines = lines.map_err(CommandError::IoError)?;

        let mut completed: usize = 0;

        let lines: Result<Vec<(String, usize)>, CommandError> = lines.into_iter().map(|line| {
            let chunks: Vec<&str> = line.split(' ').collect();

            if *chunks.get(0)
                .ok_or(CommandError::MalformedLine("expected mark sign".to_string()))? == "x" {
                completed += 1;
            }

            let line_depth = get_line_depth(&line)
                .ok_or(CommandError::MalformedLine("unable to get line depth".to_string()))?;

            Ok((line, line_depth))
        }).collect();

        let lines = lines?;

        println!("Completed tasks {}/{}", completed, lines.len());

        for (i, line) in lines.iter().enumerate() {
            let prev_depth = if i != 0 {
                match lines.get(i-1) {
                    Some(x) => x.1,
                    None => 0,
                }
            } else {
                println!();

                0
            };

            let next_depth = match lines.get(i+1) {
                Some(x) => x.1,
                None => 0,
            };

            let formatted = format_line(line, prev_depth, next_depth)
                .ok_or(CommandError::MalformedLine("unable to format line".to_string()))?;

            println!("{:0>2} {}", i+1, formatted);
        }

        Ok(())
    }

    pub fn purge(&self) -> CommandResult {
        let mut tmp_file = File::create(&self.tmp_file_path)
            .map_err(CommandError::IoError)?;

        let reader = BufReader::new(&self.file);

        for line in reader.lines() {
            let line = line
                .map_err(CommandError::IoError)?;

            let marked = line.chars().next()
                .ok_or(CommandError::MalformedLine("".to_string()))?;

            if marked != 'x' {
                writeln!(tmp_file, "{}", line)
                    .map_err(CommandError::IoError)?;
            }
        }

        fs::rename(&self.tmp_file_path, &self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn add(&self) -> CommandResult {
        let mut tmp_file = File::create(&self.tmp_file_path)
            .map_err(CommandError::IoError)?;

        let reader = BufReader::new(&self.file);

        let sub_task = self.args.get(2)
            .ok_or(CommandError::MissingArgument("expected add argument".to_string()))? == "-s";

        let mut sub_task_parent: usize = 0;
        if sub_task {
            let parent_string = self.args.get(3)
                .ok_or(CommandError::MissingArgument("expected add argument".to_string()))?;

            sub_task_parent = parent_string.parse::<usize>()
                .map_err(|_| CommandError::InvalidNumberFormat("expected add argument as number".to_string()))?;
        }

        for (i, line) in reader.lines().enumerate() {
            let line = line
                .map_err(CommandError::IoError)?;

            writeln!(tmp_file, "{}", line)
                .map_err(CommandError::IoError)?;

            if i+1 == sub_task_parent {
                let line_depth = get_line_depth(&line)
                    .ok_or(CommandError::MalformedLine("unable to get line depth".to_string()))?;

                writeln!(tmp_file, "o {} {}", line_depth+1, self.args[4..].join(" "))
                    .map_err(CommandError::IoError)?;
            }
        }

        if !sub_task {
            writeln!(tmp_file, "o 1 {}", self.args[2..].join(" "))
                .map_err(CommandError::IoError)?;
        }

        fs::rename(&self.tmp_file_path, &self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn mark(&self) -> CommandResult {
        let mut tmp_file = File::create(&self.tmp_file_path)
            .map_err(CommandError::IoError)?;

        let reader = BufReader::new(&self.file);

        let mut arg = self.args.get(2)
            .ok_or(CommandError::MissingArgument("expected mark argument".to_string()))?;

        let recursive = match arg.as_str() {
            "-r" => true,
            _ => false,
        };

        if recursive {
            arg = self.args.get(3)
                .ok_or(CommandError::MissingArgument("expected mark argument".to_string()))?;
        }

        let line_number = arg.parse::<usize>()
            .map_err(|_| CommandError::InvalidNumberFormat("expected mark argument as number".to_string()))?;

        apply_recursive(
            reader,
            line_number,
            recursive,
            &mut tmp_file,
            //active function: set a line as marked (x)
            |line: String, file: &mut File| -> CommandResult {
                let slice: Vec<&str> = line.split(' ').collect();
                let remaining = slice[1..].join(" ");
                writeln!(file, "x {}", remaining)
                    .map_err(CommandError::IoError)?;

                Ok(())
            },
            //passive function: just write the line
            |line: String, file: &mut File| -> CommandResult {
                writeln!(file, "{}", line)
                    .map_err(CommandError::IoError)?;

                Ok(())
            })?;

        fs::rename(&self.tmp_file_path, &self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn unmark(&self) -> CommandResult {
        let mut tmp_file = File::create(&self.tmp_file_path)
            .map_err(CommandError::IoError)?;

        let reader = BufReader::new(&self.file);

        let mut arg = self.args.get(2)
            .ok_or(CommandError::MissingArgument("expected unmark argument".to_string()))?;

        let recursive = match arg.as_str() {
            "-r" => true,
            _ => false,
        };

        if recursive {
            arg = self.args.get(3)
                .ok_or(CommandError::MissingArgument("expected unmark argument".to_string()))?;
        }

        let line_number = arg.parse::<usize>()
            .map_err(|_| CommandError::InvalidNumberFormat("expected unmark argument as number".to_string()))?;

        apply_recursive(
            reader,
            line_number,
            recursive,
            &mut tmp_file,
            //active function: set a line as marked (x)
            |line: String, file: &mut File| -> CommandResult {
                let slice: Vec<&str> = line.split(' ').collect();
                let remaining = slice[1..].join(" ");
                writeln!(file, "o {}", remaining)
                    .map_err(CommandError::IoError)?;

                Ok(())
            },
            //passive function: just write the line
            |line: String, file: &mut File| -> CommandResult {
                writeln!(file, "{}", line)
                    .map_err(CommandError::IoError)?;

                Ok(())
            })?;

        fs::rename(&self.tmp_file_path, &self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn clear(&self) -> CommandResult {
        File::create(&self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn remove(&self) -> CommandResult {
        let mut tmp_file = File::create(&self.tmp_file_path)
            .map_err(CommandError::IoError)?;

        let reader = BufReader::new(&self.file);

        let mut arg = self.args.get(2)
            .ok_or(CommandError::MissingArgument("expected remove argument".to_string()))?;

        let recursive = match arg.as_str() {
            "-r" => true,
            _ => false,
        };

        if recursive {
            arg = self.args.get(3)
                .ok_or(CommandError::MissingArgument("expected remove argument".to_string()))?;
        }

        let line_number = arg.parse::<usize>()
            .map_err(|_| CommandError::InvalidNumberFormat("expected remove argument as number".to_string()))?;

        apply_recursive(
            reader,
            line_number,
            recursive,
            &mut tmp_file,

            //active function (remove command so just dont write anything
            |_, _| -> CommandResult {
                Ok(())
            },

            //passive function just write the line
            |line: String, file: &mut File| -> CommandResult {
                writeln!(file, "{}", line).map_err(CommandError::IoError)?;

                Ok(())
            })?;

        fs::rename(&self.tmp_file_path, &self.file_path)
            .map_err(CommandError::IoError)?;

        Ok(())
    }

    pub fn help() -> CommandResult {
        let help_message = r#"TASC - Task Administration and Scheduling CLI

Usage:
    task <command> [options]

Commands:
    task add <text>                       Add a new task with the provided text.
    task add -s <number> <text>           Add a new sub-task under the task specified by <number>.
    task remove <number>                  Remove the task specified by <number>.
    task remove -r <number>               Recursively remove the task specified by <number> and all its sub-tasks.
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
    task remove -r 3                      Recursively removes the task with ID 3 and all related sub-tasks.
    task mark 2                           Marks the task with ID 2 as completed.
    task clear                            Permanently remove all tasks.
    task purge                            Permanently removes all completed tasks from the list."#;

        println!("{}", help_message);

        Ok(())
    }

    pub fn version() -> CommandResult {
        let package_name = env!("CARGO_PKG_NAME");
        let package_version = env!("CARGO_PKG_VERSION");

        println!("{} version {}", package_name, package_version);

        Ok(())
    }
}

fn format_line(line: &(String, usize), _prev_depth: usize, next_depth: usize) -> Option<String> {
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

fn apply_recursive(
    reader: BufReader<&File>,
    start_line: usize,
    recursive: bool,
    file: &mut File,
    active: fn (line: String, file: &mut File) -> CommandResult,
    passive: fn (line: String, file: &mut File) -> CommandResult
) -> CommandResult {
    let mut is_sub_task = false;
    let mut parent_depth: usize = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line
            .map_err(CommandError::IoError)?;

        if recursive {
            let cur_depth = get_line_depth(&line)
                .ok_or(CommandError::MalformedLine("unable to get line depth".to_string()))?;

            if is_sub_task {
                if cur_depth <= parent_depth {
                    is_sub_task = false;
                }
            } else if i+1 == start_line {
                is_sub_task = true;
                parent_depth = cur_depth;
            }
        }

        if i+1 == start_line || is_sub_task {
            active(line, file)?;
        } else {
            passive(line, file)?;
        }
    }

    Ok(())
}

fn get_line_depth(line: &String) -> Option<usize> {
    let chunks: Vec<&str> = line.split(' ').collect();
    let cur_depth_str = chunks.get(1)?;

    match cur_depth_str.parse::<usize>() {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}
