use chrono::*;
use std::io;

fn main() {
    let date = Utc::today();

    let mut day = String::new();

    let mut month = String::new();

    let mut year = String::new();

    println!("What day where you born?");

    io::stdin()
    .read_line(&mut day)
    .expect("Failed to read line");

    println!("What month where you born ?");

    io::stdin()
    .read_line(&mut month)
    .expect("Failed to read line");

    println!("What year were you born ?");

    io::stdin()
    .read_line(&mut year)
    .expect("Failed to read line");

    let day: u32 = day.trim().parse().expect("Incorrect number");

    let month: u32 = month.trim().parse().expect("Incorrect number");

    let year: i32 = year.trim().parse().expect("Incorrect number");

    let days = age_in_days(date, day, month, year);
    
    println!("Your age is {} days old", days);
}

fn age_in_days(date: Date<chrono::Utc>, day: u32, month: u32, year: i32) -> i64 {

    NaiveDate::from_ymd(date.year(), date.month(), date.day()).signed_duration_since(NaiveDate::from_ymd(year, month, day))
    .num_days()
}

