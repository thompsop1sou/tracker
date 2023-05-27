use time_tracker::date::Date;

fn main() {
    let today = Date::new_from_today();
    let tomorrow = today.add_days(1).unwrap();
    let week_hence = today.add_days(7).unwrap();
    let yesterday = today.sub_days(1).unwrap();
    let week_ago = today.sub_days(7).unwrap();
    println!("A week ago the date was {}", week_ago);
    println!("Yesterday's date is {}", yesterday);
    println!("Today's date is {}", today);
    println!("Tomorrow's date is {}", tomorrow);
    println!("A week hence the date will be {}", week_hence);
}
