use json::JsonValue;

use crate::date::Date;
use crate::activity_log::ActivityLog;

// Struct Definition
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct DayLog {
    date: Date,
    activity_logs: Vec<ActivityLog>,
}

// Public Methods
impl DayLog {
    // Creates a new, default DayLog
    pub fn new() -> DayLog {
        DayLog {
            date: Date::new(),
            activity_logs: Vec::new(),
        }
    }

    // Populates this DayLog from the data in a JsonValue object
    pub fn json_populate(self: &mut Self, parsed: &JsonValue) -> Result<(), String> {
        // Get the date
        let mut date: Date;
        if parsed.has_key("date") {
            let date_str: &str;
            match parsed["date"].as_str() {
                Some(ds) => {
                    date_str = ds;
                }
                None => {
                    return Err(String::from("JSON object cannot be interpreted as DayLog (\"date\" key is not mapped to a date string)"));
                }
            }
            match Date::new_from_string(date_str) {
                Ok(d) => {
                    date = d;
                }
                Err(_) => {
                    return Err(String::from("JSON object cannot be interpreted as DayLog (\"date\" key is not mapped to a date string)"));
                }
            }
        } else {
            return Err(String::from("JSON object cannot be interpreted as DayLog (no \"date\" key)"));
        }
        // Get the activity logs
        let mut activity_logs: Vec<ActivityLog> = Vec::new();
        if parsed.has_key("activity logs") {
            for act_log_obj in parsed["activity logs"].members() {
                let mut act_log = ActivityLog::new();
                match act_log.json_populate(act_log_obj) {
                    Ok(_) => activity_logs.push(act_log),
                    Err(_) => continue,
                }
            }
        } else {
            return Err(String::from("JSON object cannot be interpreted as DayLog (no \"activity logs\" key)"));
        }
        // Populate the struct
        self.date = date;
        self.activity_logs = activity_logs;
        Ok(())
    }

    // Returns a JSON formatted string representation of this DayLog
    pub fn to_string(self: &Self, tab: &str) -> String {
        let mut activity_logs_string = String::new();
        if self.activity_logs.len() > 0 {
            activity_logs_string.push_str("[\n");
            for i in 0..self.activity_logs.len() {
                let activity_log_string = self.activity_logs[i].to_string(tab);
                let lines: Vec<&str> = activity_log_string.lines().collect();
                for j in 0..lines.len() {
                    let new_line: String;
                    if j == lines.len() - 1 && i < self.activity_logs.len() - 1 {
                        new_line = format!("{}{}{},\n", tab, tab, lines[j]);
                    } else {
                        new_line = format!("{}{}{}\n", tab, tab, lines[j]);
                    }
                    activity_logs_string.push_str(&new_line);
                }
            }
            activity_logs_string.push_str(tab);
            activity_logs_string.push_str("]");
        } else {
            activity_logs_string.push_str("[]");
        }
        format!("{{\n{}\"date\": \"{}\",\n{}\"activity logs\": {}\n}}", tab, self.date.to_string(), tab, activity_logs_string)
    }
}