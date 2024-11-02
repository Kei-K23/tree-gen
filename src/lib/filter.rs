use std::{
    fs::{self, metadata},
    path::Path,
    time::UNIX_EPOCH,
};

use super::date::parse_date;

/// Check if a directory contains files with the specified extension (for filtering)
pub fn contains_matching_files_extension(
    path: &Path,
    file_extension: Option<&String>,
    ignore_hidden: bool,
) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();

            // Ignore hidden files/folders if `ignore_hidden` is enabled
            if ignore_hidden && file_name.starts_with('.') {
                continue;
            }

            // If it’s a file, check for extension match
            if path.is_file() {
                if let Some(ext) = file_extension {
                    if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                        return true; // Found a matching file
                    }
                } else {
                    return true; // No extension filter, any file counts
                }
            }

            // If it’s a directory, recursively check inside
            if path.is_dir()
                && contains_matching_files_extension(&path, file_extension, ignore_hidden)
            {
                return true; // Matching files found in a subdirectory
            }
        }
    }
    false // No matching files found
}

pub fn apply_date_filter(path: &Path, filter: &str) -> bool {
    if let Ok(metadata) = metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let modified_date = modified
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            let filter_parts: Vec<&str> = filter.split_whitespace().collect();

            match filter_parts.as_slice() {
                ["before", filter_date] => modified_date < parse_date(filter_date),
                ["after", filter_date] => modified_date > parse_date(filter_date),
                ["between", date1, date2] => {
                    modified_date > parse_date(date1) && modified_date < parse_date(date2)
                }
                _ => true,
            }
        } else {
            true
        }
    } else {
        true
    }
}
