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
        self.from_json(&parsed).unwrap_or_else(|_| {});
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
    pub fn add(self: &mut Self, date: Date, activity: String, minutes: u16) -> Result<(), String> {
        // If we already have the date, update the data
        if self.data.contains_key(&date) {
            let mut activities = self.data[&date].clone();
            if self.data[&date].contains_key(&activity) {
                let total_minutes = self.data[&date][&activity] + minutes;
                if total_minutes > 60 * 24 {
                    return Err(format!("Add error: total minutes exceeds {} for {} on {}", 60 * 24, activity, date.to_string()));
                }
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
    }

    // Subtracts minutes from an activity on a date
    pub fn subtract(self: &mut Self, date: Date, activity: String, minutes: u16) -> Result<(), String> {
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
                return Err(format!("Subtract error: no minutes recorded for {} on {}", activity, date.to_string()));
            }
        // If we don't already have the date, let the user know
        } else {
            return Err(format!("Subtract error: no activities recorded for {}", date.to_string()));
        }
        Ok(())
    }

    // Returns a summary (as a String) of the activities for a given date or date range
    pub fn summarize(self: &Self, start_date: Date, end_date: Date) -> Result<String, String> {
        // Make sure start_date is before end_date
        if end_date < start_date {
            return Err(format!("Summarize error: end date {} is before start date {}", end_date.to_string(), start_date.to_string()));
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
            let mut summary = format!("Summary from {} to {}:\n\n", start_date.to_string(), end_date.to_string());
            summary.push_str("ACTIVITY\tTOTAL TIME\tAVG TIME\n");
            for (activity, minutes) in activities {
                let line: String;
                let activity_tab = if activity.len() < 8 {"\t\t"} else {"\t"};
                let minutes_tab = if minutes.to_string().len() < 8 {"\t\t"} else {"\t"};
                line = format!("{}{}{}{}{}\n", activity, activity_tab, minutes, minutes_tab, minutes/num_days);
                summary.push_str(&line);
            }
            Ok(summary.trim_end_matches("\n").to_string())
        // If there is not data for those dates, return an error indicating that
        } else {
            Err(format!("Summarize error: no data for {} to {}", start_date.to_string(), end_date.to_string()))
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

#[cfg(test)]
mod tests {
    use crate::tracker_data::TrackerData;
    use crate::date::Date;

    #[test]
    fn from_to_json() {
        // Create valid data
        let valid_json = json::object!{
            "2023-2-1": json::object!{
                guitar: 30,
                school: 180
            },
            "2023-3-1": json::object!{
                school: 210,
                work: 120
            }
        };
        let mut tracker_data = TrackerData::new();
        // Use from_json and to_json methods on this data
        assert_eq!(tracker_data.from_json(&valid_json),
                    Ok(()));
        assert_eq!(tracker_data.to_json(),
                    Ok(valid_json.clone()));

        // Create invalid data
        let invalid_json = json::object!{
            "2023-2-1": json::object!{
                guitar: "hello"
            },
            "2023.3.1": json::object!{
                school: 210,
                work: 120
            },
            "2023-4-1": json::array![
                "school",
                60,
                "guitar",
                45
            ]
        };
        // Use from_json on this data (should return error without changing the data)
        assert_eq!(tracker_data.from_json(&invalid_json),
                    Err(String::from("From JSON error: JSON cannot be interpreted as TrackerData")));
        assert_eq!(tracker_data.to_json(),
                    Ok(valid_json.clone()));
    }

    #[test]
    fn add() {
        // Create a base TrackerData object to work with
        let valid_json = json::object!{
            "2023-2-1": json::object!{
                guitar: 30,
                school: 180
            },
            "2023-3-1": json::object!{
                school: 210,
                work: 120
            }
        };
        let mut tracker_data = TrackerData::new();
        assert_eq!(tracker_data.from_json(&valid_json), Ok(()));

        // Add time to an existing activity
        let date = Date::new_from_string("2023-2-1").unwrap();
        assert_eq!(tracker_data.add(date.clone(), String::from("guitar"), 15),
                    Ok(()));
        assert_eq!(tracker_data.data[&date]["guitar"],
                    45);

        // Add a new activity on an existing date
        assert_eq!(tracker_data.add(date.clone(), String::from("work"), 180),
                    Ok(()));
        assert_eq!(tracker_data.data[&date]["work"],
                    180);

        // Add a new activity on a new date
        let new_date = Date::new_from_string("2023-4-1").unwrap();
        assert_eq!(tracker_data.add(new_date.clone(), String::from("school"), 60),
                    Ok(()));
        assert_eq!(tracker_data.data[&new_date]["school"],
                    60);

        // Try to add more than a day's worth of time to an activity
        assert_eq!(tracker_data.add(new_date.clone(), String::from("school"), 1440),
                    Err(String::from("Add error: total minutes exceeds 1440 for school on 2023-4-1")));
    }

    #[test]
    fn subtract() {
        // Create a base TrackerData object to work with
        let valid_json = json::object!{
            "2023-2-1": json::object!{
                guitar: 30,
                school: 180
            },
            "2023-3-1": json::object!{
                school: 210,
                work: 120
            }
        };
        let mut tracker_data = TrackerData::new();
        assert_eq!(tracker_data.from_json(&valid_json), Ok(()));

        // Subtract time from an existing activity
        let date = Date::new_from_string("2023-2-1").unwrap();
        assert_eq!(tracker_data.subtract(date.clone(), String::from("guitar"), 15),
                    Ok(()));
        assert_eq!(tracker_data.data[&date]["guitar"],
                    15);

        // Subtract all time from an existing activity (it should no longer exist in the TrackerData)
        assert_eq!(tracker_data.subtract(date.clone(), String::from("school"), 200),
                    Ok(()));
        assert_eq!(tracker_data.data[&date].contains_key(&String::from("school")),
                    false);

        // Try to subtract time from a non-existent activity on an existing date
        assert_eq!(tracker_data.subtract(date.clone(), String::from("work"), 30),
                    Err(String::from("Subtract error: no minutes recorded for work on 2023-2-1")));

        // Try to subtract time from a non-existent activity on a non-existent date
        let new_date = Date::new_from_string("2023-4-1").unwrap();
        assert_eq!(tracker_data.subtract(new_date.clone(), String::from("work"), 30),
                    Err(String::from("Subtract error: no activities recorded for 2023-4-1")));
    }

    #[test]
    fn summarize() {
        // Create a TrackerData object to work with
        let valid_json = json::object!{
            "2023-2-1": json::object!{
                guitar: 30,
                school: 180
            },
            "2023-3-1": json::object!{
                school: 210,
            }
        };
        let mut tracker_data = TrackerData::new();
        assert_eq!(tracker_data.from_json(&valid_json), Ok(()));

        // Summarize a valid date range
        let date1 = Date::new_from_string("2023-2-1").unwrap();
        let date2 = Date::new_from_string("2023-4-1").unwrap();
        let summary1 = "Summary from 2023-2-1 to 2023-4-1:

ACTIVITY\tTOTAL TIME\tAVG TIME
guitar\t\t30\t\t15
school\t\t390\t\t195";
        let summary2 = "Summary from 2023-2-1 to 2023-4-1:

ACTIVITY\tTOTAL TIME\tAVG TIME
school\t\t390\t\t195
guitar\t\t30\t\t15";
        let summarize_result = tracker_data.summarize(date1.clone(), date2.clone());
        assert!(summarize_result.clone() == Ok(String::from(summary1)) ||
                summarize_result.clone() == Ok(String::from(summary2)));

        // Try to summarize an invalid date range
        assert_eq!(tracker_data.summarize(date2.clone(), date1.clone()),
                    Err(String::from("Summarize error: end date 2023-2-1 is before start date 2023-4-1")));
        
        // Try to summarize a date range with no data
        let date3 = Date::new_from_string("2023-5-1").unwrap();
        assert_eq!(tracker_data.summarize(date2.clone(), date3.clone()),
                    Err(String::from("Summarize error: no data for 2023-4-1 to 2023-5-1")));
    }
}