use std::env;
use std::process;
use tracker::tracker_data::TrackerData;
use tracker::date::Date;

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
            // Parse the arguments
            let (date, activity, minutes) = parse_add_sub_args(other_args).unwrap_or_else(|e| {
                print_error_and_exit(&e);
                (Date::new(), String::new(), 0)
            });
            // Call the add method on tracker_data
            tracker_data.add(date, activity, minutes).unwrap_or_else(|e| {
                print_error_and_exit(&e);
            });
        }
        // Remove time from an activity
        "sub" => {
            // Parse the arguments
            let (date, activity, minutes) = parse_add_sub_args(other_args).unwrap_or_else(|e| {
                print_error_and_exit(&e);
                (Date::new(), String::new(), 0)
            });
            // Call the subtract method on tracker_data
            tracker_data.subtract(date, activity, minutes).unwrap_or_else(|e| {
                print_error_and_exit(&e);
            });
        }
        // Print a summary of a date range
        "sum" => {
            // Parse the arguments
            let (start_date, end_date) = parse_sum_args(other_args).unwrap_or_else(|e| {
                print_error_and_exit(&e);
                (Date::new(), Date::new())
            });
            // Call the summarize method on tracker_data
            let summary = tracker_data.summarize(start_date, end_date).unwrap_or_else(|e| {
                print_error_and_exit(&e);
                String::new()
            });
            println!("{}", summary);
        }
        // Asking for help, print out instructions
        "?" | "/?" | "--help" | "help" => {
            print_instructions();
        }
        // Invalid function argument, print instructions
        c => {
            print_error_and_exit(&format!("Parse arguments error: \"{c}\" not a valid function (use \"help\" to see a list of valid functions)"))
        }
    }

    // Save the tracker log into the JSON file
    tracker_data.save_to_file(&filename)
        .unwrap_or_else(|e| print_error_and_exit(&e));
}

// Parse arguments into values needed for add and sub functions
fn parse_add_sub_args(other_args: Vec<String>) -> Result<(Date, String, u16), String> {
    let date: Date;
    let activity: String;
    let minutes: u16;
    if other_args.len() >= 3 {
        match Date::new_from_string(&other_args[0]) {
            Ok(d) => date = d,
            Err(_) => return Err(format!("Parse arguments error: \"{}\" cannot be interpreted as a date", other_args[0])),
        }
        activity = other_args[1].clone();
        match other_args[2].parse::<u16>() {
            Ok(m) => minutes = m,
            Err(_) => return Err(format!("Parse arguments error: \"{}\" cannot be interpreted as an integer", other_args[2])),
        }
    } else {
        return Err(String::from("Parse arguments error: not enough arguments for \"add\" or \"sub\" function"));
    }
    Ok((date, activity, minutes))
}

// Parse arguments into values needed for sum function
fn parse_sum_args(other_args: Vec<String>) -> Result<(Date, Date), String> {
    let start_date: Date;
    let end_date: Date;
    match other_args.len() {
        0 => {
            return Err(String::from("Parse arguments error: not enough arguments for \"sum\" function"));
        }
        1 => {
            match Date::new_from_string(&other_args[0]) {
                Ok(d) => start_date = d,
                Err(_) => return Err(format!("Parse arguments error: \"{}\" cannot be interpreted as a date", other_args[0])),
            }
            end_date = start_date.clone();
        }
        _ => {
            match Date::new_from_string(&other_args[0]) {
                Ok(d) => start_date = d,
                Err(_) => return Err(format!("Parse arguments error: \"{}\" cannot be interpreted as a date", other_args[0])),
            }
            match Date::new_from_string(&other_args[1]) {
                Ok(d) => end_date = d,
                Err(_) => return Err(format!("Parse arguments error: \"{}\" cannot be interpreted as a date", other_args[1])),
            }
        }
    }
    Ok((start_date, end_date))
}

// Print the error message to standard error and exit the process
fn print_error_and_exit(error_msg: &str) {
    eprintln!("{}", error_msg);
    process::exit(1);
}

// Print the instructions to standard output
fn print_instructions() {
    let mut instr = String::new();
    instr.push_str("Valid tracker functions:\n\n");
    instr.push_str("FUNCTION <ARGUMENT>                DESCRIPTION\n");
    instr.push_str("add <date> <activity> <minutes>    add minutes to an activity on a date\n");
    instr.push_str("sub <date> <activity> <minutes>    subtract minutes from an activity on a date\n");
    instr.push_str("sum <date>                         print summary of activities on a date\n");
    instr.push_str("sum <start_date> <end_date>        print summary of activities from start date to end date\n");
    instr.push_str("\nNote: Date arguments should use one of the following formats:\n");
    instr.push_str("    <year>-<month>-<day> (e.g. 2023-5-31)\n");
    instr.push_str("    today (gives today's date)\n");
    instr.push_str("    today-<n> (gives a date n days before today)\n");
    instr.push_str("    today+<n> (gives a date n days after today)");
    println!("{}", instr);
}