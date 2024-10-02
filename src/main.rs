use std::env;
use std::path::Path;
use std::fs::File;
use std::fs;
/*
fn list(file: &File) -> Result<(),> {


    Ok(())
}
*/
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

    file;

    match cmd {
        "list" => {},
        "mark" => {},
        "add" => {},
        "remove" => {},
        "clean" => {},
        _ => panic!("invalid args"),
    }
}
