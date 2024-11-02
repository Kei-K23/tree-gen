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
