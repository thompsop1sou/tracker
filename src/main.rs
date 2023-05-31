use std::env;
use std::process;
use time_tracker::tracker_data::TrackerData;

fn main() {
    // Open the JSON file and load into the tracker data
    let filename = "tracker_data.json";
    let mut tracker_data = TrackerData::new();
    tracker_data.load_from_file(&filename)
        .unwrap_or_else(|e| print_error_and_exit(&e));

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
        // Add time to an activity
        "add" => {
            tracker_data.add(other_args)
                .unwrap_or_else(|e| print_error_and_exit(&e));
        }
        // Remove time from an activity
        "sub" => {
            tracker_data.subtract(other_args)
                .unwrap_or_else(|e| print_error_and_exit(&e));
        }
        // Print a summary of a date range
        "sum" => {
            let summary = tracker_data.summarize(other_args).unwrap_or_else(|e| {
                print_error_and_exit(&e);
                String::new()
            });
            println!("{}", summary);
        }
        // Invalid command line arguments, print instructions
        _ => {
            print_instr_and_exit();
        }
    }

    // Save the tracker log into the JSON file
    tracker_data.save_to_file(&filename)
        .unwrap_or_else(|e| print_error_and_exit(&e));
}

// Function prints the error message to standard error and then exits the process
fn print_error_and_exit(error_msg: &str) {
    eprintln!("{}", error_msg);
    process::exit(1);
}

// Function prints the instructions to standard error and then exits the process
fn print_instr_and_exit() {
    let mut instr = String::new();
    instr.push_str("Please enter a valid command:\n");
    instr.push_str("    add time to an activity         \"add <date> <activity> <minutes>\"\n");
    instr.push_str("    subtract time from an activity  \"sub <date> <activity> <minutes>\"\n");
    instr.push_str("    print summary of a date         \"sum <date>\"");
    instr.push_str("    print summary of a date range   \"sum <start_date> <end_date>\"");
    eprintln!("{}", instr);
    process::exit(1);
}