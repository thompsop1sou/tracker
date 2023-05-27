use chrono::Local;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u16,
    day: u16,
}

impl fmt::Display for Date {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Date {
    // Create a new Date with default values year: 2000, month: 1, day: 1
    pub fn new() -> Date {
        Date{year: 2000, month: 1, day: 1}
    }

    // Create a new Date with today's date
    pub fn today() -> Date {
        let today = Local::now().date_naive();
        Date::from_string(&today.format("%Y-%m-%d").to_string()).unwrap()
    }

    // Create a new Date from integer arguments for year, month, and day
    pub fn from_ints(year: u16, month: u16, day: u16) -> Result<Date, &'static str> {
        let mut date = Date::new();
        date.set_year(year)?;
        date.set_month(month)?;
        date.set_day(day)?;
        Ok(date)
    }

    // Create a new Date from a string argument formatted like "year.month.day"
    pub fn from_string(date_str: &str) -> Result<Date, &'static str> {
        let parts: Vec<&str> = date_str.split("-").collect();
        if parts.len() == 3 {
            let year: u16;
            match parts[0].parse::<u16>() {
                Ok(x) => year = x,
                Err(_) => return Err("Date parse error: cannot parse year"),
            }
            let month: u16;
            match parts[1].parse::<u16>() {
                Ok(x) => month = x,
                Err(_) => return Err("Date parse error: cannot parse month"),
            }
            let day: u16;
            match parts[2].parse::<u16>() {
                Ok(x) => day = x,
                Err(_) => return Err("Date parse error: cannot parse day"),
            }
            Date::from_ints(year, month, day)
        } else {
            Err("Date parse error: incorrect number of seperators")
        }
    }

    // Get a string representation of this Date
    pub fn to_string(self: &Self) -> String {
        format!("{}-{}-{}", self.year.to_string(),self.month.to_string(), self.day.to_string())
    }

    pub fn set_year(self: &mut Self, year: u16) -> Result<(), &'static str> {
        self.year = year;
        Ok(())
    }

    pub fn get_year(self: &Self) -> u16 {
        self.year
    }

    pub fn set_month(self: &mut Self, month: u16) -> Result<(), &'static str> {
        if month < 1 {
            Err("Set month error: month too small")
        } else if month > 12 {
            Err("Set month error: month too large")
        } else {
            self.month = month;
            Ok(())
        }
    }

    pub fn get_month(self: &Self) -> u16 {
        self.month
    }

    pub fn set_day(self: &mut Self, day: u16) -> Result<(), &'static str> {
        if day < 1 {
            Err("Set day error: day too small")
        } else if day > 31 {
            Err("Set day error: day too large")
        } else {
            self.day = day;
            Ok(())
        }
    }

    pub fn get_day(self: &Self) -> u16 {
        self.day
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;

    #[test]
    fn from_string() {
        assert_eq!(Date::from_string("2012-11-21"),
                    Ok(Date::from_ints(2012, 11, 21).unwrap()));
        assert_eq!(Date::from_string("2030-1"),
                    Err("Date parse error: incorrect number of seperators"));
        assert_eq!(Date::from_string("20"),
                    Err("Date parse error: incorrect number of seperators"));
        assert_eq!(Date::from_string("2030-1-3-4"),
                    Err("Date parse error: incorrect number of seperators"));
        assert_eq!(Date::from_string("e-1-4"),
                    Err("Date parse error: cannot parse year"));
        assert_eq!(Date::from_string("2021-e-4"),
                    Err("Date parse error: cannot parse month"));
        assert_eq!(Date::from_string("2021-1-e"),
                    Err("Date parse error: cannot parse day"));
    }

    #[test]
    fn to_string() {
        assert_eq!(Date::new().to_string(),
                    String::from("2000-1-1"));
        assert_eq!(Date::from_ints(2023, 5, 27).unwrap().to_string(),
                    String::from("2023-5-27"));
    }

    #[test]
    fn setters() {
        let mut date = Date::new();
        assert_eq!(date.set_month(0), Err("Set month error: month too small"));
        assert_eq!(date.get_month(), 1);
        assert_eq!(date.set_month(13), Err("Set month error: month too large"));
        assert_eq!(date.get_month(), 1);
        assert_eq!(date.set_month(6), Ok(()));
        assert_eq!(date.get_month(), 6);
        assert_eq!(date.set_day(0), Err("Set day error: day too small"));
        assert_eq!(date.get_day(), 1);
        assert_eq!(date.set_day(32), Err("Set day error: day too large"));
        assert_eq!(date.get_day(), 1);
        assert_eq!(date.set_day(15), Ok(()));
        assert_eq!(date.get_day(), 15);
    }
}