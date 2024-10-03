use std::{env, io, fs};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn format_line(line: String) -> Option<String> {
    let mut formatted = String::new();
    let mut chars = line.chars();

    match chars.next() {
        Some('x') => formatted.push_str("[x] "),
        Some('o') => formatted.push_str("[ ] "),
        _ => return None,
    };

    if chars.next().is_none() {
        return None;
    }

    formatted.extend(chars);

    Some(formatted)
}

fn list(file: File) -> Result<(), io::Error> {
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                match format_line(l) {
                    Some(str) => println!("{:0>2} {}", i+1, str),
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line")),
                };
            },
            Err(r) => return Err(r),
        }
    }

    Ok(())
}

fn purge(file: File, file_path: &Path, tmp_file_path: &Path) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let marked = match l.chars().next() {
                    Some(x) => x == 'x',
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line")),
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

fn add(file: File, file_path: &Path, tmp_file_path: &Path,  args: Vec<String>) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    let sub_task = match args.get(2) {
        Some(x) => x == "-s",
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args")),
    };

    let mut sub_task_parent: usize = 0;
    if sub_task {
        let parent_string = match args.get(3) {
            Some(x) => x,
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args")),
        };

        sub_task_parent = match parent_string.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
        }
    }

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(l) => {
                writeln!(tmp_file, "{}", l)?;
                if i+1 == sub_task_parent {
                    let chunks: Vec<&str> = l.split(' ').collect();
                    let line_depth = match chunks.get(1) {
                        Some(&x) => x.chars().count(),
                        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
                    };

                    writeln!(tmp_file, "o {} {}", "─".repeat(line_depth+1), args[4..].join(" "))?;
                }
            },
            Err(r) => return Err(r),
        }
    }

    if !sub_task {
        writeln!(tmp_file, "o ─ {}", args[2..].join(" "))?;
    }

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

fn mark(file: File, file_path: &Path, tmp_file_path: &Path, args: Vec<String>) -> Result<(), io::Error> {
    let mut tmp_file = File::create(tmp_file_path)?;
    let reader = BufReader::new(&file);

    let line_number = match args.get(2) {
        Some(x) => x,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid args")),
    };

    let line_number = match line_number.parse::<usize>() {
        Ok(num) => num,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
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

fn clear(file: File, file_path: &Path, tmp_file_path: &Path) -> Result<(), io::Error> {
    let tmp_file = File::create(tmp_file_path)?;

    drop(file);
    drop(tmp_file);

    fs::rename(tmp_file_path, file_path)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = match args.get(1) {
        Some(x) => x.as_str(),
        None => panic!("no args provided"),
    };

    let home_path = "HOME";

    let path = match env::var(home_path) {
        Ok(val) => val + "/.tasc/data.tasc" ,
        Err(e) => panic!("{}", e),
    };
    let path = Path::new(&path);

    let parent_path = match path.parent() {
        Some(parent) => parent,
        None => panic!("invalid parent path")
    };

    if !parent_path.exists() && fs::create_dir(parent_path).is_err() {
        panic!("error on creating dir: {}", parent_path.to_str().unwrap());
    }

    let tmp_path = parent_path.join("tmp");
    let tmp_path = tmp_path.as_path();

    let file = if path.exists() {
        File::open(path)
    } else {
        File::create(path)
    };

    let file = match file {
        Ok(f) => f,
        Err(r) => panic!("{}", r),
    };

    let result = match cmd {
        "list" => list(file),
        "add" => add(file, path, tmp_path, args),
        "mark" => mark(file, path, tmp_path, args),
        "purge" => purge(file, path, tmp_path),
        "clear" => clear(file, path, tmp_path),
        "remove" => Ok(()),
        _ => panic!("invalid args"),
    };


    match result {
        Ok(_) => (),
        Err(r) => panic!("{}", r),
    }
}
