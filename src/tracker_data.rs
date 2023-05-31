use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use json::JsonValue;

use crate::date::Date;

// Struct Definition
pub struct TrackerData {
    data: HashMap<Date, HashMap<String, f32>>
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

    pub fn add(self: &mut Self, other_args: Vec<String>) -> Result<(), String> {
        Ok(())
    }

    pub fn remove(self: &mut Self, other_args: Vec<String>) -> Result<(), String> {
        Ok(())
    }

    pub fn summarize(self: &Self, other_args: Vec<String>) -> Result<(), String> {
        Ok(())
    }
}

// Private Methods
impl TrackerData {
    // Fills this TrackerData with the date from the JsonValue object
    fn from_json(self: &mut Self, tracker_json: &JsonValue) -> Result<(), String> {
        let mut new_data: HashMap<Date, HashMap<String, f32>> = HashMap::new();
        // Loop over all the key-value pairs in parsed
        for (date_str, activities_json) in tracker_json.entries() {
            // Get the date from the key
            let date: Date;
            match Date::new_from_string(date_str) {
                Ok(d) => date = d,
                Err(_) => continue,
            }
            // Get the activities from the value, which is itself a HashMap
            let mut activities: HashMap<String, f32> = HashMap::new();
            // Loop over all the key-value pairs in activities_json
            for (act_str, dur) in activities_json.entries() {
                // Get the activity from the key
                let activity = String::from(act_str);
                // Get the duration from the value
                let duration: f32;
                match dur.as_f32() {
                    Some(d) => duration = d,
                    None => continue,
                }
                // Add activity and duration as a key-value pair to activities
                activities.insert(activity, duration);
            }
            // Add date and activities as a key-value pair to new_data
            new_data.insert(date, activities);
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
            for (activity, duration) in activities {
                match activities_json.insert(&activity, *duration) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(String::from("To JSON error: TrackerData cannot be interpreted as JSON"))
                    }
                }
            }
            // Insert the date_string and activity_json into tracker_json as a key-value pair
            match tracker_json.insert(&date_string, activities_json) {
                Ok(_) => {}
                Err(_) => {
                    return Err(String::from("To JSON error: TrackerData cannot be interpreted as JSON"))
                }
            }
        }
        Ok(tracker_json)
    }
}