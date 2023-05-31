use json::JsonValue;

// Struct Definition
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ActivityLog {
    category: String,
    duration: f32,
}

// Public Methods
impl ActivityLog {
    // Creates a new, default ActivityLog
    pub fn new() -> ActivityLog {
        ActivityLog {
            category: String::new(),
            duration: 0.0,
        }
    }

    // Populates this ActivityLog from the data in a JsonValue object
    pub fn json_populate(self: &mut Self, parsed: &JsonValue) -> Result<(), String> {
        // Get the category
        let mut category: String;
        if parsed.has_key("category") {
            match parsed["category"].as_str() {
                Some(cs) => {
                    category = cs.to_string();
                }
                None => {
                    return Err(String::from("JSON object cannot be interpreted as ActivityLog (\"category\" key is not mapped to a string)"));
                }
            }
        } else {
            return Err(String::from("JSON object cannot be interpreted as ActivityLog (no \"category\" key)"));
        }
        // Get the duration
        let mut duration: f32;
        if parsed.has_key("duration") {
            match parsed["duration"].as_f32() {
                Some(d) => {
                    duration = d;
                }
                None => {
                    return Err(String::from("JSON object cannot be interpreted as ActivityLog (\"duration\" key is not mapped to a u8)"));
                }
            }
        } else {
            return Err(String::from("JSON object cannot be interpreted as ActivityLog (no \"duration\" key)"));
        }
        // Populate the struct
        self.category = category;
        self.duration = duration;
        Ok(())
    }

    // Returns a JSON formatted string representation of this DayLog
    pub fn to_string(self: &Self, tab: &str) -> String {
        format!("{{\n{}\"category\": \"{}\",\n{}\"duration\": {}\n}}", tab, self.category, tab, self.duration)
    }
}