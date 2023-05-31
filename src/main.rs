use std::env;
use time_tracker::tracker_log::TrackerLog;

fn main() {
    // Open the JSON file and load into the tracker log
    let filename = "time_tracker.json";
    let mut tracker_log = TrackerLog::new();
    match tracker_log.load_from_file(&filename) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    // Get command line arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut func_arg = "";
    let mut other_args = Vec::new();
    if args.len() > 0 {
        func_arg = &*args[0];
        for a in &args[1..] {
            other_args.push(a.clone());
        }
    }

    // Run a function depending on the command line arguments
    match func_arg {
        "add" => add(other_args),
        "log" => log(other_args),
        "sum" => sum(other_args),
        _ => println!("Please enter a valid command (\"add\", \"log\", or \"sum\")..."),
    }

    // Save the tracker log into the JSON file
    match tracker_log.save_to_file(&filename) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
}

fn add(other_args: Vec<String>) {
    println!("Adding {:?}...", other_args)
}

fn log(other_args: Vec<String>) {
    println!("Logging {:?}...", other_args)
}

fn sum(other_args: Vec<String>) {
    println!("Summing {:?}...", other_args)
}