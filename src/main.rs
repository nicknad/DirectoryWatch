use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::SystemTime;

pub fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Oh, no! No Arguments given!");
    }

    let directory = Path::new(&args[1]);
    // compare time to file metadata and 
    // if changes occur update time and execute commands from args
    let mut point_in_time = SytemTime::now();
    traverse_dir(&directory);

    if args.len() == 2 {
        panic!("No commands given");
    }

    for cmd in &args[2..] {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd ])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to execute process")
        };
    }
}

fn traverse_dir(dir: &Path) -> io::Result<()>{
    for entry in fs::read_dir(dir)? {

        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let _ = traverse_dir(&path);
        } else {
            println!("{}", path.file_name().unwrap().to_string_lossy().to_string());
        }
    }

    Ok(())
}
