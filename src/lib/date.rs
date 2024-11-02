use std::{
    fs::metadata,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_human_readable_date(path: &Path) -> String {
    match metadata(path) {
        Ok(metadata) => match metadata.modified() {
            Ok(time) => {
                // Calculate the duration since UNIX_EPOCH
                let duration = time
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");

                // Convert the duration to seconds and nanoseconds
                let secs = duration.as_secs();
                let nanos = duration.subsec_nanos();

                // Convert seconds to a standard date-time
                let datetime = UNIX_EPOCH + std::time::Duration::new(secs, nanos);
                let datetime = datetime_to_readable(datetime);

                datetime
            }
            Err(_) => "modified date unknown".to_string(),
        },
        Err(_) => "modified date unknown".to_string(),
    }
}

pub fn datetime_to_readable(system_time: SystemTime) -> String {
    // Convert `SystemTime` to a UNIX timestamp (seconds and nanoseconds)
    let duration = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let secs = duration.as_secs() as i64;

    // Extract individual date and time components
    let year = 1970 + secs / 31_536_000; // rough approximation of years from UNIX epoch
    let month = ((secs % 31_536_000) / 2_592_000) + 1; // Months from seconds
    let day = ((secs % 2_592_000) / 86_400) + 1; // Days from seconds
    let hour = (secs % 86_400) / 3_600; // Hours from seconds
    let minute = (secs % 3_600) / 60;
    let second = secs % 60;

    // Format as a readable string
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    )
}

/// Parses a date in "YYYY-MM-DD" format to a timestamp.
pub fn parse_date(date: &str) -> u64 {
    // Split date components from "YYYY-MM-DD" format
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        panic!("Invalid date format, expected YYYY-MM-DD");
    }

    // Parse year, month, and day as integers
    let year: i32 = parts[0].parse().expect("Invalid year");
    let month: u32 = parts[1].parse().expect("Invalid month");
    let day: u32 = parts[2].parse().expect("Invalid day");

    // Validate and calculate the number of days since the UNIX epoch
    let days_since_epoch =
        days_since_unix_epoch(year, month, day).expect("Invalid date or calculation error");

    // Convert days to seconds and return as timestamp
    (days_since_epoch * 86400) as u64
}

/// Calculates the number of days since the UNIX epoch (1970-01-01) for a given date.
fn days_since_unix_epoch(year: i32, month: u32, day: u32) -> Option<i64> {
    // Define constants
    const UNIX_EPOCH_YEAR: i32 = 1970;
    const DAYS_IN_COMMON_YEAR: i64 = 365;
    const DAYS_IN_LEAP_YEAR: i64 = 366;

    // Check for valid date ranges
    if year < UNIX_EPOCH_YEAR || month == 0 || month > 12 || day == 0 || day > 31 {
        return None;
    }

    // Determine the total number of days from 1970-01-01 to the given date
    let mut days = 0;

    for y in UNIX_EPOCH_YEAR..year {
        days += if is_leap_year(y) {
            DAYS_IN_LEAP_YEAR
        } else {
            DAYS_IN_COMMON_YEAR
        };
    }

    // Add the days for the given month and day in the target year
    days += days_in_months(year, month - 1) + day as i64 - 1;

    Some(days)
}

/// Checks if a year is a leap year.
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns the number of days from January to the end of the previous month.
fn days_in_months(year: i32, month: u32) -> i64 {
    const DAYS_IN_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days = 0;
    for i in 0..month as usize {
        days += DAYS_IN_MONTH[i];
        // Add a day for February if it's a leap year
        if i == 1 && is_leap_year(year) {
            days += 1;
        }
    }
    days
}
