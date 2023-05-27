use time_tracker::date::Date;

fn main() {
    let mut date = Date::today();
    println!("Today's date is {}", date);
    date.set_day(date.get_day() + 1).expect("Couldn't add a day to today.");
    println!("Tomorrow's date is {}", date);
}
