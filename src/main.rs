use std::{env, io, fs};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn list(file: &File) -> Result<(), io::Error> {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                match format_line(l) {
                    Some(str) => println!("{}", str),
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "malformed line")),
                };
            },
            Err(r) => return Err(r),
        }
    }

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
        "list" => list(&file),
        "mark" => Ok(()),
        "add" => Ok(()),
        "remove" => Ok(()),
        "clean" => Ok(()),
        _ => panic!("invalid args"),
    };

    match result {
        Ok(_) => (),
        Err(r) => panic!("{}", r),
    }
}
