use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use json::JsonValue;

use crate::date::Date;

// Struct Definition
pub struct TrackerData {
    data: HashMap<Date, HashMap<String, u16>>
}

// Public Methods
impl TrackerData {
    // Creates a new, default TrackerData
    pub fn new() -> TrackerData {
        TrackerData {
            data: HashMap::new()
        }
    }

    // Loads JSON formatted data from file filename into this TrackerData
    pub fn load_from_file(self: &mut Self, filename: &str) -> Result<(), String> {
        // Open the file and get the contents
        let mut contents = String::new();
        match File::open(filename) {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(_) => {return Err(format!("Load from file error: cannot read \"{filename}\""));}
                }
            },
            Err(_) => {}
        }
        // Parse the contents into a JsonValue object
        let parsed: json::JsonValue;
        if contents == "" {
            parsed = JsonValue::new_object();
        } else {
            match json::parse(&contents) {
                Ok(p) => {
                    parsed = p;
                }
                Err(_) => {
                    return Err(format!("Load from file error: cannot parse contents of \"{filename}\""));
                }
            }
        }
        // Use the JsonValue object to populate this TrackerData struct
        match self.from_json(&parsed) {
            Ok(_) => (),
            Err(_) => (), // Error can be ignored, just means there was no data in parsed
        }
        Ok(())
    }

    // Saves JSON formatted data from this TrackerData into the file filename 
    pub fn save_to_file(self: &mut Self, filename: &str) -> Result<(), String> {
        // Open the file for writing
        match File::create(filename) {
            Ok(mut file) => {
                // Write the JSON to the file
                let tracker_json = self.to_json()?;
                match file.write_all(tracker_json.pretty(4).as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!("Save to file error: cannot write to \"{filename}\""));
                    }
                }
            }
            Err(_) => {
                return Err(format!("Save to file error: cannot open \"{filename}\""));
            }
        }
        Ok(())
    }

    // Adds minutes to an activity on a date
    pub fn add(self: &mut Self, args: Vec<String>) -> Result<(), String> {
        // If we have the correct number of arguments...
        if args.len() == 3 {
            // Parse the arguments
            let date = Date::new_from_string(&args[0])?;
            let activity = args[1].clone();
            let minutes: u16;
            match args[2].parse::<u16>() {
                Ok(m) => minutes = m,
                Err(_) => return Err(String::from("Add activity error: minutes argument cannot be interpreted as number")),
            }
            // If we already have the date, update the data
            if self.data.contains_key(&date) {
                let mut activities = self.data[&date].clone();
                if self.data[&date].contains_key(&activity) {
                    let total_minutes = self.data[&date][&activity] + minutes;
                    activities.insert(activity, total_minutes);
                } else {
                    activities.insert(activity, minutes);
                }
                self.data.insert(date, activities);
            // If we don't already have the date, create the data
            } else {
                let mut activities: HashMap<String, u16> = HashMap::new();
                activities.insert(activity, minutes);
                self.data.insert(date, activities);
            }
            Ok(())
        // If we don't have the correct number of arguments...
        } else {
            Err(String::from("Add activity error: incorrect number of arguments"))
        }
    }

    // Subtracts minutes from an activity on a date
    pub fn subtract(self: &mut Self, args: Vec<String>) -> Result<(), String> {
        // If we have the correct number of arguments...
        if args.len() == 3 {
            // Parse the arguments
            let date = Date::new_from_string(&args[0])?;
            let activity = args[1].clone();
            let minutes: u16;
            match args[2].parse::<u16>() {
                Ok(m) => minutes = m,
                Err(_) => return Err(String::from("Subtract activity error: minutes argument cannot be interpreted as number")),
            }
            // If we already have the date, update the data
            if self.data.contains_key(&date) {
                // If we already have the activity, update the data
                if self.data[&date].contains_key(&activity) {
                    let mut activities = self.data[&date].clone();
                    if minutes < self.data[&date][&activity] {
                        let total_minutes = self.data[&date][&activity] - minutes;
                        activities.insert(activity, total_minutes);
                    } else {
                        activities.remove(&activity);
                    }
                    if activities.is_empty() {
                        self.data.remove(&date);
                    } else {
                        self.data.insert(date, activities);
                    }
                // If we don't already have the activity, let the user know
                } else {
                    return Err(String::from("Subtract activity error: no minutes recorded for that activity on that date"));
                }
            // If we don't already have the date, let the user know
            } else {
                return Err(String::from("Subtract activity error: no activities recorded for that date"));
            }
            Ok(())
        // If we don't have the correct number of arguments...
        } else {
            Err(String::from("Subtract activity error: incorrect number of arguments"))
        }
    }

    // Returns a summary (as a String) of the activities for a given date or date range
    pub fn summarize(self: &Self, args: Vec<String>) -> Result<String, String> {
        // If we only have one date...
        if args.len() == 1{
            // Parse the argument
            let date = Date::new_from_string(&args[0])?;
            // If there is data for that date, return a string representing that data
            if self.data.contains_key(&date) && !self.data[&date].is_empty() {
                let mut summary = format!("ACTIVITY\tTOTAL TIME\n");
                for (activity, minutes) in &self.data[&date] {
                    let line: String;
                    if activity.len() < 8 {
                        line = format!("{activity}\t\t{minutes}\n");
                    } else {
                        line = format!("{activity}\t{minutes}\n");
                    }
                    summary.push_str(&line);
                }
                Ok(summary.trim_end_matches("\n").to_string())
            // If there is not data for that date, return a string indicating that
            } else {
                Ok(format!("There is no data for {}", date.to_string()))
            }
        // If we have two dates (for a range)...
        } else if args.len() == 2 {
            // Parse the arguments
            let start_date = Date::new_from_string(&args[0])?;
            let end_date = Date::new_from_string(&args[1])?;
            if start_date > end_date {
                return Err(String::from("Summarize error: end date is before start date"));
            }
            // Collect the data from those dates
            let mut num_days: u16 = 0;
            let mut activities: HashMap<String, u16> = HashMap::new();
            let mut curr_date = start_date.clone();
            while curr_date <= end_date {
                if self.data.contains_key(&curr_date) {
                    for (activity, minutes) in &self.data[&curr_date] {
                        if activities.contains_key(activity) {
                            activities.insert(activity.clone(), activities[activity] + *minutes);
                        } else {
                            activities.insert(activity.clone(), *minutes);
                        }
                    }
                    num_days += 1;
                }
                curr_date = curr_date.add_days(1).unwrap();
            }
            // If there is data for those dates, return a string representing that data
            if !activities.is_empty() {
                let mut summary = format!("ACTIVITY\tTOTAL TIME\tAVG TIME\n");
                for (activity, minutes) in activities {
                    let line: String;
                    if activity.len() < 8 {
                        line = format!("{}\t\t{}\t\t{}\n", activity, minutes, minutes/num_days);
                    } else {
                        line = format!("{}\t{}\t\t{}\n", activity, minutes, minutes/num_days);
                    }
                    summary.push_str(&line);
                }
                Ok(summary.trim_end_matches("\n").to_string())
            // If there is not data for those dates, return a string indicating that
            } else {
                Ok(format!("There is no data for {} to {}", start_date.to_string(), end_date.to_string()))
            }
        // If we don't have the correct number of arguments...
        } else {
            Err(String::from("Summarize error: incorrect number of arguments"))
        }
    }
}

// Private Methods
impl TrackerData {
    // Fills this TrackerData with the date from the JsonValue object
    fn from_json(self: &mut Self, tracker_json: &JsonValue) -> Result<(), String> {
        let mut new_data: HashMap<Date, HashMap<String, u16>> = HashMap::new();
        // Loop over all the key-value pairs in parsed
        for (date_str, activities_json) in tracker_json.entries() {
            // Get the date from the key
            let date: Date;
            match Date::new_from_string(date_str) {
                Ok(d) => date = d,
                Err(_) => continue,
            }
            // Get the activities from the value, which is itself a HashMap
            let mut activities: HashMap<String, u16> = HashMap::new();
            // Loop over all the key-value pairs in activities_json
            for (act_str, dur) in activities_json.entries() {
                // Get the activity from the key
                let activity = String::from(act_str);
                // Get the minutes from the value
                let minutes: u16;
                match dur.as_u16() {
                    Some(m) => minutes = m,
                    None => continue,
                }
                // Add activity and minutes as a key-value pair to activities
                activities.insert(activity, minutes);
            }
            // Add date and activities as a key-value pair to new_data
            if !activities.is_empty() {
                new_data.insert(date, activities);
            }
        }
        // Check to make sure we actually got some new data
        if new_data.is_empty() {
            Err(String::from("From JSON error: JSON cannot be interpreted as TrackerData"))
        } else {
            self.data = new_data;
            Ok(())
        }
    }

    // Returns a JsonValue object representing this TrackerData
    fn to_json(self: &Self) -> Result<JsonValue, String> {
        let mut tracker_json = JsonValue::new_object();
        for (date, activities) in &self.data {
            // Get the string version of the date
            let date_string = date.to_string();
            // Get the JSON version of the activities
            let mut activities_json = JsonValue::new_object();
            for (activity, minutes) in activities {
                match activities_json.insert(&activity, *minutes) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("To JSON error: TrackerData cannot be interpreted as JSON"))
                    }
                }
            }
            // Insert the date_string and activity_json into tracker_json as a key-value pair
            if activities_json.len() > 0 {
                match tracker_json.insert(&date_string, activities_json) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("To JSON error: TrackerData cannot be interpreted as JSON"))
                    }
                }
            }
        }
        Ok(tracker_json)
    }
}