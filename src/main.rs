use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
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

        if cfg!(target_os = "windows") {
            execute_on_windows(&args[2..]);
        } else {
            execute_on_linux(&args[2..])
        }


    }
}

fn execute_on_linux(cmds: &[String]) {
    for cmd in cmds {
        let output = Command::new(cmd)
            .output()
            .expect("Failed to execute command");

        print_output(output);
    }
}

fn execute_on_windows(cmds: &[String]) {
    for cmd in cmds {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        print_output(output);
    }
}

fn print_output(output: Output) {
    println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));
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
