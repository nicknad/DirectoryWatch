use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};

// Expects following input via arguments
// executable name (rust default)
// path to directory
// cmds1..n
pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let number_of_args = args.len();

    if number_of_args == 1 {
        return Err(anyhow!("Oh, no! No arguments given!"));
    }

    if number_of_args == 2 {
        return Err(anyhow!("No commands given"));
    }

    let directory = Path::new(&args[1]);
    let mut point_in_time = SystemTime::now();

    loop {
        thread::sleep(Duration::from_millis(500));
        let files_changed = traverse_dir(&directory, &point_in_time)?;

        if !files_changed {
            continue;
        }

        println!("file changed");
        point_in_time = SystemTime::now();
        for cmd in &args[2..] {
            let parts: Vec<&str> = cmd.split(" ").collect();

            if parts.len() == 1 {
                let output = Command::new(cmd)
                    .output()
                    .expect("Failed to execute command!");
                println!("{:?}", String::from_utf8_lossy(&output.stdout));
            } else {
                let output = Command::new(parts[0])
                    .args(&parts[1..])
                    .output()
                    .expect("Failed to execute command!");
                println!("{:?}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}

fn traverse_dir(dir: &Path, point_in_time: &SystemTime) -> Result<bool> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let has_changed_file = traverse_dir(&path, &point_in_time)?;

            if has_changed_file {
                return Ok(true);
            }
        } else if path.is_file() {
            let meta = path.metadata()?;
            let mod_time = meta.modified()?;
            if *point_in_time < mod_time {
                return Ok(true);
            }
        }
    }

    return Ok(false);
}
