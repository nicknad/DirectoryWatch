use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, Duration};
use std::thread;

// Expects following input via arguments
// executable name (rust default)
// path to directory
// cmds1..n
pub fn main() -> Result<()> {
    let args : Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err(anyhow!("Oh, no! No arguments given!"));
    }

    if args.len() == 2 {
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
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", cmd ])
                    .output()?;

            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()?;
            };
        }
    }
}

fn traverse_dir(dir: &Path, curr_time: &SystemTime) -> Result<bool> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let has_changed_file = traverse_dir(&path, &curr_time)?;

            if has_changed_file {
                return Ok(true);
            }

        } else if path.is_file() {

            let meta = path.metadata()?;
            let mod_time = meta.modified()?;
            if mod_time > *curr_time {
                return Ok(true);
            }
        }
    }

    return Ok(false);
}
