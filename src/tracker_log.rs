use std::fs::File;
use std::io::prelude::*;
use json::JsonValue;

use crate::day_log::DayLog;

/***************/
/* Tracker Log */
/***************/

// Struct Definition
pub struct TrackerLog {
    categories: Vec<String>,
    day_logs: Vec<DayLog>,
}

// Public Methods
impl TrackerLog {
    // Creates a new, default TrackerLog
    pub fn new() -> TrackerLog {
        TrackerLog {
            categories: Vec::new(),
            day_logs: Vec::new(),
        }
    }

    // Loads JSON formatted data from file filename into this TrackerLog
    pub fn load_from_file(self: &mut Self, filename: &str) -> Result<(), String> {
        // Open the file and get the contents
        let mut contents = String::new();
        match File::open(filename) {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(_) => {return Err(format!("Something went wrong reading from \"{filename}\"..."));}
                }
            },
            Err(_) => {}
        }
        // Parse the contents into a JsonValue object
        let parsed: json::JsonValue;
        if contents == "" {
            parsed = json::object![];
        } else {
            match json::parse(&contents) {
                Ok(p) => {
                    parsed = p;
                }
                Err(_) => {
                    return Err(format!("Something went wrong parsing the contents of \"{filename}\"..."));
                }
            }
        }
        // Use the JsonValue object to populate this TrackerLog struct
        self.json_populate(&parsed)
    }
    
    // Saves JSON formatted data from this TrackerLog into the file filename 
    pub fn save_to_file(self: &mut Self, filename: &str) -> Result<(), String> {
        // Write to the file
        match File::create(filename) {
            Ok(mut file) => {
                match file.write_all(self.to_string("    ").as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!("Something went wrong writing to \"{filename}\"..."));
                    }
                }
            }
            Err(_) => {
                return Err(format!("Something went wrong creating \"{filename}\"..."));
            }
        }
        Ok(())
    }
}

// Private Methods
impl TrackerLog {
    // Populates this TrackerLog from the data in a JsonValue object
    fn json_populate(self: &mut Self, parsed: &JsonValue) -> Result<(), String> {
        // Populate the categories
        if parsed.has_key("categories") {
            for cat_obj in parsed["categories"].members() {
                match cat_obj.as_str() {
                    Some(cat_str) => self.categories.push(cat_str.to_string()),
                    None => continue,
                }
            }
        }
        // Populate the day logs
        if parsed.has_key("day logs") {
            for day_log_obj in parsed["day logs"].members() {
                let mut day_log = DayLog::new();
                match day_log.json_populate(day_log_obj) {
                    Ok(_) => self.day_logs.push(day_log),
                    Err(_) => continue,
                }
            }
        }
        Ok(())
    }

    // Returns a JSON formatted string representation of this TrackerLog
    fn to_string(self: &Self, tab: &str) -> String {
        // Construct the categories string
        let mut categories_string = String::new();
        if self.categories.len() > 0 {
            categories_string.push_str("[\n");
            for i in 0..self.categories.len() {
                let category_string = format!("{}{}\"{}\"", tab, tab, self.categories[i]);
                categories_string.push_str(&category_string);
                if i < self.categories.len() - 1 {
                    categories_string.push_str(",\n");
                }
            }
            let cats_end_string = format!("\n{}]", tab);
            categories_string.push_str(&cats_end_string);
        } else {
            categories_string.push_str("[]");
        }
        // Construct the day logs string
        let mut day_logs_string = String::new();
        if self.day_logs.len() > 0 {
            day_logs_string.push_str("[\n");
            for i in 0..self.day_logs.len() {
                let day_log_string = self.day_logs[i].to_string(tab);
                let lines: Vec<&str> = day_log_string.lines().collect();
                for j in 0..lines.len() {
                    let new_line: String;
                    if j == lines.len() - 1 && i < self.day_logs.len() - 1 {
                        new_line = format!("{}{}{},\n", tab, tab, lines[j]);
                    } else {
                        new_line = format!("{}{}{}\n", tab, tab, lines[j]);
                    }
                    day_logs_string.push_str(&new_line);
                }
            }
            day_logs_string.push_str(tab);
            day_logs_string.push_str("]");
        } else {
            day_logs_string.push_str("[]");
        }
        // Collect them together and return
        format!("{{\n{}\"categories\": {},\n{}\"day logs\": {}\n}}", tab, categories_string, tab, day_logs_string)
    }

    // Need a function to ensure there aren't any categories in an activity log which don't show up in the categories vector
}