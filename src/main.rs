// Import `NaiveDate` and `Datelike` from the `chrono` crate
use chrono::{NaiveDate, Datelike};

// Import `io` and `Write` for handling input/output
use std::io::{self, Write};

fn main() {

    print!("Enter a date (dd/mm/yyyy): ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the newline character and split the input into day, month, and year
    let date_parts: Vec<&str> = input.trim().split('/').collect();

    if date_parts.len() == 3 {
        // Parse the day, month, and year as integers
        let day: u32 = date_parts[0].parse().expect("Invalid day");
        let month: u32 = date_parts[1].parse().expect("Invalid month");
        let year: i32 = date_parts[2].parse().expect("Invalid year");

        // Create a NaiveDate object with the provided input using from_ymd_opt
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => {
                // Get the day of the week
                let weekday = date.weekday();
                // Print the day of the week
                println!("The day on {}/{}/{} was a {}", day, month, year, weekday);
                // println!("The day on given date was a {}",weekday);
            }
            None => {
                println!("Invalid date provided. Please ensure the date is valid.");
            }
        }
    } else {
        println!("Please enter a valid date in the format dd/mm/yyyy.");
    }
}
