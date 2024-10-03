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

    formatted.push_str(&chunks[2..].join(" "));

    Some(formatted)
}

fn list(file: File) -> Result<(), io::Error> {
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
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
        };

        let line_depth = match line_depth {
            Ok(num) => num,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
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
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line")),
        };
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
                        Some(&x) => x.parse::<usize>(),
                        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
                    };

                    let line_depth = match line_depth {
                        Ok(num) => num,
                        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid number format")),
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

fn remove(file: File, file_path: &Path, tmp_file_path: &Path, args: Vec<String>) -> Result<(), io::Error> {
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
        "remove" => remove(file, path, tmp_path, args),
        "purge" => purge(file, path, tmp_path),
        "clear" => clear(file, path, tmp_path),
        _ => panic!("invalid args"),
    };


    match result {
        Ok(_) => (),
        Err(r) => panic!("{}", r),
    }
}
