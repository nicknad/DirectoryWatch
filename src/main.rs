use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Oh, no! No Arguments given!");
    }

    let directory = Path::new(&args[1]);
    // compare time to file metadata and 
    // if changes occur update time and execute commands from args
    let mut point_in_time = SystemTime::now();
    print_time(&point_in_time);


    let result = traverse_dir(&directory);

    if !result.is_ok() {
        panic!("traverse failed")
    }

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
    point_in_time = SystemTime::now();
    print_time(&point_in_time);
}

fn print_time(time: &SystemTime) {
    let since_the_epoch = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    println!("{:?}", in_ms);
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
