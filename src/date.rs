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
    // Create a new Date with default values (year: 2000, month: 1, day: 1)
    pub fn new() -> Date {
        Date{year: 2000, month: 1, day: 1}
    }

    // Create a new Date with today's date
    pub fn new_from_today() -> Date {
        let today = Local::now().date_naive();
        Date::new_from_string(&today.format("%Y-%m-%d").to_string()).unwrap()
    }

    // Create a new Date from integer arguments for year, month, and day
    pub fn new_from_ints(year: u16, month: u16, day: u16) -> Result<Date, &'static str> {
        let mut date = Date::new();
        date.set_from_ints(year, month, day)?;
        Ok(date)
    }

    // Create a new Date from a string argument formatted like "year-month-day"
    pub fn new_from_string(date_str: &str) -> Result<Date, &'static str> {
        let mut date = Date::new();
        date.set_from_string(date_str)?;
        Ok(date)
    }

    // Set this Date from integer arguments for year, month, and day
    pub fn set_from_ints(self: &mut Self, year: u16, month: u16, day: u16) -> Result<(), &'static str> {
        self.set_year(year)?;
        self.set_month(month)?;
        self.set_day(day)?;
        Ok(())
    }

    // Set this Date from a string argument formatted like "year-month-day"
    pub fn set_from_string(self: &mut Self, date_str: &str) -> Result<(), &'static str> {
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
            self.set_from_ints(year, month, day)
        } else {
            Err("Date parse error: incorrect number of seperators")
        }
    }

    // Get a string representation of this Date
    pub fn to_string(self: &Self) -> String {
        format!("{}-{}-{}", self.year.to_string(),self.month.to_string(), self.day.to_string())
    }

    // Get a tuple representation of this Date
    pub fn to_tuple(self: &Self) -> (u16, u16, u16) {
        (self.year, self.month, self.day)
    }

    fn set_year(self: &mut Self, year: u16) -> Result<(), &'static str> {
        self.year = year;
        Ok(())
    }

    fn set_month(self: &mut Self, month: u16) -> Result<(), &'static str> {
        if month < 1 {
            Err("Set date error: month too small")
        } else if month > 12 {
            Err("Set date error: month too large")
        } else {
            self.month = month;
            Ok(())
        }
    }

    fn set_day(self: &mut Self, day: u16) -> Result<(), &'static str> {
        let is_leap_year = self.year % 4 == 0 && !(self.year % 100 == 0 && !(self.year % 400 == 0));
    
        let days_in_month = match self.month {
            4 | 6 | 9 | 11 =>  30,
            2 => if is_leap_year {29} else {28},
            _ => 31,
        };
    
        if day < 1 {
            Err("Set date error: day too small")
        } else if day > days_in_month {
            Err("Set date error: day too large")
        } else {
            self.day = day;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;

    #[test]
    fn parsing() {
        // Can be parsed
        assert_eq!(Date::new_from_string("2012-11-21"),
                    Ok(Date::new_from_ints(2012, 11, 21).unwrap()));
        // Can't be parsed because of separators
        assert_eq!(Date::new_from_string("2030-1"),
                    Err("Date parse error: incorrect number of seperators"));
        assert_eq!(Date::new_from_string("20"),
                    Err("Date parse error: incorrect number of seperators"));
        assert_eq!(Date::new_from_string("2030-1-3-4"),
                    Err("Date parse error: incorrect number of seperators"));
        // Can't be parsed because of non-integers
        assert_eq!(Date::new_from_string("e-1-4"),
                    Err("Date parse error: cannot parse year"));
        assert_eq!(Date::new_from_string("2021-e-4"),
                    Err("Date parse error: cannot parse month"));
        assert_eq!(Date::new_from_string("2021-1-e"),
                    Err("Date parse error: cannot parse day"));
    }

    #[test]
    fn representing() {
        let date1 = Date::new();
        let date2 = Date::new_from_ints(2023, 5, 27).unwrap();
        // Representing as string
        assert_eq!(date1.to_string(), String::from("2000-1-1"));
        assert_eq!(date2.to_string(), String::from("2023-5-27"));
        // Representing as tuple
        assert_eq!(date1.to_tuple(), (2000, 1, 1));
        assert_eq!(date2.to_tuple(), (2023, 5, 27));
    }

    #[test]
    fn setting() {
        let mut date = Date::new();
        // Setting month
        assert_eq!(date.set_from_ints(2000, 1, 15), Ok(()));
        assert_eq!(date.set_from_ints(2000, 0, 15), Err("Set date error: month too small"));
        assert_eq!(date.set_from_ints(2000, 12, 15), Ok(()));
        assert_eq!(date.set_from_ints(2000, 13, 15), Err("Set date error: month too large"));
        // Setting day in January (31 days)
        assert_eq!(date.set_from_ints(2000, 1, 1), Ok(()));
        assert_eq!(date.set_from_ints(2000, 1, 0), Err("Set date error: day too small"));
        assert_eq!(date.set_from_ints(2000, 1, 31), Ok(()));
        assert_eq!(date.set_from_ints(2000, 1, 32), Err("Set date error: day too large"));
        // Setting day in April (30 days)
        assert_eq!(date.set_from_ints(2000, 4, 1), Ok(()));
        assert_eq!(date.set_from_ints(2000, 4, 0), Err("Set date error: day too small"));
        assert_eq!(date.set_from_ints(2000, 4, 30), Ok(()));
        assert_eq!(date.set_from_ints(2000, 4, 31), Err("Set date error: day too large"));
        // Setting day in February (normal: 28 days, leap year: 29 days)
        assert_eq!(date.set_from_ints(2000, 2, 1), Ok(()));
        assert_eq!(date.set_from_ints(2000, 2, 0), Err("Set date error: day too small"));
        assert_eq!(date.set_from_ints(2000, 2, 29), Ok(()));
        assert_eq!(date.set_from_ints(2000, 2, 30), Err("Set date error: day too large"));
        assert_eq!(date.set_from_ints(2001, 2, 28), Ok(()));
        assert_eq!(date.set_from_ints(2001, 2, 29), Err("Set date error: day too large"));
        assert_eq!(date.set_from_ints(2004, 2, 29), Ok(()));
        assert_eq!(date.set_from_ints(2004, 2, 30), Err("Set date error: day too large"));
        assert_eq!(date.set_from_ints(2100, 2, 28), Ok(()));
        assert_eq!(date.set_from_ints(2100, 2, 29), Err("Set date error: day too large"));
    }
}