use std::{
    fs::{self, metadata},
    path::Path,
};

use crate::lib::filter::contains_matching_files_extension;

/// Generate an ASCII representation of the directory structure.
pub fn generate_tree(
    path: &Path,
    prefix: &str,
    file_extension: Option<&String>,
    depth: usize,
    max_depth: Option<usize>,
    ignore_hidden: bool,
    show_size: bool,
) {
    // Stop when reach to max depth
    if let Some(max) = max_depth {
        if depth > max {
            return; // Exit the function
        }
    }

    if let Ok(entries) = fs::read_dir(path) {
        // Check inside directory

        let entries: Vec<_> = entries.filter_map(Result::ok).collect();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();

            // If ignore_hidden is true, then skip all file that start with .
            if ignore_hidden && file_name.starts_with(".") {
                continue;
            }

            // Check file extension when file extension have value
            // This is check for directory for file extension
            if path.is_dir() && file_extension.is_some() {
                if !contains_matching_files_extension(&path, file_extension, ignore_hidden) {
                    continue;
                }
            }

            // This is check directly for a file
            if let Some(ext) = file_extension {
                if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some(ext) {
                    continue;
                }
            }

            // If show size flags is true, then get the file size from metadata
            let size_str = if show_size {
                match metadata(&path) {
                    // Convert to KB by divided by 1024
                    Ok(metadata) => format!(" ({:.2} KB)", metadata.len() as f64 / 1024.0),
                    Err(_) => String::from(" (size unknown)"),
                }
            } else {
                String::new()
            };

            let is_last = i == entries.len() - 1;
            let new_prefix = if is_last { "└── " } else { "├── " };
            println!("{}{}{}{}", prefix, new_prefix, file_name, size_str);

            // If path is dir, then recurse into directories
            if path.is_dir() {
                let additional_prefix = if is_last { "    " } else { "│   " };
                generate_tree(
                    &path,
                    &format!("{}{}", prefix, additional_prefix),
                    file_extension,
                    depth + 1,
                    max_depth,
                    ignore_hidden,
                    show_size,
                );
            }
        }
    }
}
