use std::{env, fs};
use std::path::Path;
use std::fs::File;

mod commands;
use commands::{list, add, mark, remove, clear, purge, help, version};


fn handle_error(msg: &str, exit_code: i32) -> () {
    eprintln!("Error: {}", msg);
    std::process::exit(exit_code);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = match args.get(1) {
        Some(x) => x.as_str(),
        None => return handle_error("invalid command. Use '--help' to see the list of available commands.", 1),
    };

    let home_path = "HOME";

    let path = match env::var(home_path) {
        Ok(val) => val + "/.tasc/data.tasc" ,
        Err(_) => return handle_error("invalid $HOME env var.", 2),
    };
    let path = Path::new(&path);

    let parent_path = match path.parent() {
        Some(parent) => parent,
        None => return handle_error("invalid parent path ($HOME/.tasc).", 3),
    };

    if !parent_path.exists() && fs::create_dir(parent_path).is_err() {
        handle_error(&format!("creating dir: {}", parent_path.to_str().unwrap()), 4);
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
        Err(r) => return handle_error(&format!("unable to open file {}.",r), 5),
    };

    let result = match cmd {
        "l" | "list" => list(file),
        "add" => add(file, path, tmp_path, args),
        "mark" => mark(file, path, tmp_path, args),
        "remove" | "rm" => remove(file, path, tmp_path, args),
        "purge" => purge(file, path, tmp_path),
        "clear" => clear(file, path, tmp_path),
        "-h" | "--help" | "help" => Ok(help()),
        "-v" | "--version" => Ok(version()),
        _ => panic!("invalid args"),
    };


    match result {
        Ok(_) => (),
        Err(e) => handle_error(&e.to_string(), 6),
    }
}
