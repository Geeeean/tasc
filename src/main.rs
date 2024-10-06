use std::{env, fs};
use std::path::PathBuf;
use std::fs::File;

mod command;
use command::Command;

mod error;
use error::CommandError;

fn handle_error(msg: &str, exit_code: i32) -> () {
    eprintln!("Error, {}", msg);
    eprintln!("Use '--help' to see the list of available commands.");
    std::process::exit(exit_code);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = match args.get(1) {
        Some(x) => x.as_str(),
        None => return handle_error("not enough args.", 1),
    };

    #[cfg(target_os = "windows")]
    let home_path = "USERPROFILE";

    #[cfg(target_os = "linux")]
    let home_path = "HOME";

    #[cfg(target_os = "macos")]
    let home_path = "HOME";

    let path = match env::var(home_path) {
        Ok(val) => val + "/.tasc/data.tasc" ,
        Err(_) => return handle_error("invalid $HOME env var.", 2),
    };
    let path = PathBuf::from(&path);

    let parent_path = match path.parent() {
        Some(parent) => parent,
        None => return handle_error("invalid parent path ($HOME/.tasc).", 3),
    };

    if !parent_path.exists() && fs::create_dir(parent_path).is_err() {
        handle_error(&format!("creating dir: {}", parent_path.to_str().unwrap()), 4);
    }

    let tmp_path = parent_path.join("tmp");

    let file = if path.exists() {
        File::open(&path)
    } else {
        File::create(&path)
    };

    let file = match file {
        Ok(f) => f,
        Err(r) => return handle_error(&format!("unable to open file {}.",r), 5),
    };

    let command = Command::new(file, path, tmp_path, args.clone());

    let result = match cmd {
        "l" | "list" => command.list(),
        "add" => command.add(),
        "mark" => command.mark(),
        "unmark" => command.unmark(),
        "remove" | "rm" => command.remove(),
        "purge" => command.purge(),
        "clear" => command.clear(),
        "-h" | "--help" | "help" => Command::help(),
        "-v" | "--version" => Command::version(),
        _ => Err(CommandError::MissingArgument("invalid command".to_string())),
    };


    match result {
        Ok(_) => (),
        Err(e) => handle_error(&e.to_string(), 6),
    }
}
