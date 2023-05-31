use std::env;
use std::process;
use time_tracker::tracker_log::TrackerLog;

fn main() {
    // Open the JSON file and load into the tracker log
    let filename = "time_tracker.json";
    let mut tracker_log = TrackerLog::new();
    tracker_log.load_from_file(&filename)
        .unwrap_or_else(|e| print_error_and_exit(e));

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
        "add-cat" => tracker_log.add_category(other_args)
                        .unwrap_or_else(|e| print_error_and_exit(e)),
        "del-cat" => tracker_log.delete_category(other_args)
                        .unwrap_or_else(|e| print_error_and_exit(e)),
        "add-log" => tracker_log.add_log(other_args)
                        .unwrap_or_else(|e| print_error_and_exit(e)),
        "del-log" => tracker_log.delete_log(other_args)
                        .unwrap_or_else(|e| print_error_and_exit(e)),
        "sum" => tracker_log.summarize(other_args)
                        .unwrap_or_else(|e| print_error_and_exit(e)),
        _ => print_instructions_and_exit(),
    }

    // Save the tracker log into the JSON file
    tracker_log.save_to_file(&filename)
        .unwrap_or_else(|e| print_error_and_exit(e));
}

fn print_error_and_exit(error_msg: String) {
    eprintln!("{}", error_msg);
    process::exit(1);
}

fn print_instructions_and_exit() {
    let mut instructions = String::new();
    instructions.push_str("Please enter a valid command:\n");
    instructions.push_str("    add a category     \"add-cat <category>\"\n");
    instructions.push_str("    delete a category  \"del-cat <category>\"\n");
    instructions.push_str("    add a log          \"add-log <date> <category> <duration>\"\n");
    instructions.push_str("    delete a log       \"del-log <date> <category>\"\n");
    instructions.push_str("    print a summary    \"sum <start_date> <end_date>\"");
    eprintln!("{}", instructions);
    process::exit(1);
}