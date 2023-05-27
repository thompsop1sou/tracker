use time_tracker::date::Date;

fn main() {
    let today = Date::new_from_today();
    println!("Today's date is {}", today);
}
