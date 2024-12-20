use std::{
    fs::{self, metadata, File},
    io::{self, BufRead},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use colored::Colorize;
use regex::Regex;
use serde::Serialize;

use crate::lib::filter::contains_matching_files_extension;

use super::{
    date::get_human_readable_date, filter::apply_date_filter, icon::get_file_icon,
    output_file::write_output,
};

#[derive(Serialize)]
pub struct TreeNode {
    name: String,
    size: String,
    node_type: String,
    permission: String,
    last_modification_date: String,
    children: Vec<TreeNode>,
}

/// Generate an ASCII representation of the directory structure.
pub fn generate_tree(
    path: &Path,
    prefix: &str,
    file_extension: Option<&String>,
    output_file: Option<&String>,
    depth: usize,
    max_depth: Option<usize>,
    ignore_hidden: bool,
    show_size: bool,
    branch_style: Option<&String>,
    preview_lines: Option<&String>,
    date_filter: Option<&String>,
    size_min: Option<u64>,
    size_max: Option<u64>,
    include: Option<&String>,
    exclude: Option<&String>,
    icons: bool,
) {
    // Determine branch style based on style
    let (branch, last_branch, continuation, close_line) = match branch_style {
        Some(branch_style) => match branch_style.as_str() {
            "ascii" => ("|-- ", "`-- ", "|   ", "|"), // ASCII style
            _ => ("├── ", "└── ", "│   ", "│"),       // Unicode style (default)
        },
        None => ("├── ", "└── ", "│   ", "│"), // Unicode style (default)
    };

    // Stop when reach to max depth
    if let Some(max) = max_depth {
        if depth > max {
            return; // Exit the function
        }
    }

    if let Ok(entries) = fs::read_dir(path) {
        // Check inside directory
        // Collect entries and apply filtering
        let entries: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();

                // Skip hidden files if ignore_hidden is set
                if ignore_hidden && file_name.starts_with('.') {
                    return false;
                }

                if path.is_file() {
                    // Filter by file extension if provided
                    if let Some(ext) = file_extension {
                        if path.extension().and_then(|e| e.to_str()) != Some(ext) {
                            return false;
                        }
                    }

                    // Filter by file size
                    if let Some(size) = metadata(&path).map(|meta| meta.len()).ok() {
                        if let Some(min) = size_min {
                            if size < min {
                                return false;
                            }
                        }
                        if let Some(max) = size_max {
                            if size > max {
                                return false;
                            }
                        }
                    }

                    // Filter by include/exclude patterns if provided
                    if let Some(include_pattern) = include {
                        let re = Regex::new(include_pattern).unwrap();
                        if !re.is_match(&file_name) {
                            return false;
                        }
                    }
                    if let Some(exclude_pattern) = exclude {
                        let re = Regex::new(exclude_pattern).unwrap();
                        if re.is_match(&file_name) {
                            return false;
                        }
                    }

                    // Filter by date if provided
                    if let Some(date_filter) = date_filter {
                        if !apply_date_filter(&path, date_filter) {
                            return false;
                        }
                    }
                }

                // If all checks passed, include the entry
                true
            })
            .collect();

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

            // This is check for a file
            if let Some(ext) = file_extension {
                if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some(ext) {
                    continue;
                }
            }

            // If size_min and size_max flags parse, then check and filter files by file size
            // Corrected size filter logic
            if let Some(size) = metadata(&path).map(|meta| meta.len()).ok() {
                // TODO! Handle for ending character
                if path.is_file() {
                    if let Some(min) = size_min {
                        if size < min {
                            continue;
                        }
                    }
                    if let Some(max) = size_max {
                        if size > max {
                            continue;
                        }
                    }
                }
            }

            if path.is_file() {
                // If include flag parse, then filter by file name with regex pattern
                if let Some(include_pattern) = include {
                    let re = Regex::new(&include_pattern).unwrap();
                    if !re.is_match(&file_name) {
                        continue;
                    }
                }

                // If exclude flag parse, then filter by file name with regex pattern
                if let Some(exclude_pattern) = exclude {
                    let re = Regex::new(&exclude_pattern).unwrap();
                    if re.is_match(&file_name) {
                        continue;
                    }
                }
            }

            // If date filter flag parse, the check and filter by date
            if let Some(date_filter) = date_filter {
                if !apply_date_filter(&path, date_filter) {
                    continue;
                }
            }

            // If show size flags is true, then get the file size from metadata
            let size_str = if show_size {
                if path.is_dir() {
                    format!(" ({:.2} KB)", get_directory_size(&path) as f64 / 1024.0)
                } else {
                    match metadata(&path) {
                        // Convert to KB by divided by 1024
                        Ok(metadata) => format!(" ({:.2} KB)", metadata.len() as f64 / 1024.0),
                        Err(_) => String::from(" (size unknown)"),
                    }
                }
            } else {
                String::new()
            };

            let is_last = i == entries.len() - 1;
            let new_prefix = if is_last { last_branch } else { branch };

            // Use color for better visualization
            let file_name_colored = if path.is_dir() {
                file_name.green()
            } else {
                file_name.normal()
            };

            // Combine icon and file name
            let display_name = match icons {
                true => {
                    // Get the icon for file and folder
                    let icon = get_file_icon(&path);
                    format!("{} {}", icon, file_name_colored)
                }
                false => format!("{}", file_name_colored),
            };

            let content = format!("{}{}{}{}", prefix, new_prefix, display_name, size_str);

            // If output file exist, then write to file instead of printing out to terminal
            if let Some(output) = output_file {
                write_output(&output, &content).expect("Failed to write to file");
            } else {
                println!("{}", content);
            }

            // If preview lines flag parse and current path is a file, then show preview content of file
            if path.is_file() {
                if let Some(num_lines_str) = preview_lines {
                    let preview_prefix = if is_last {
                        format!("{}     ", prefix)
                    } else {
                        format!("{}|    ", prefix)
                    };

                    let num_lines = num_lines_str.parse::<usize>().unwrap();

                    if let Ok(file) = File::open(&path) {
                        let reader = io::BufReader::new(file);
                        for (line_num, line) in reader.lines().enumerate() {
                            // When reach to preview line number, then break the loop
                            if line_num >= num_lines {
                                break;
                            }
                            match line {
                                Ok(content) => println!("{}{}", preview_prefix, content),
                                Err(_) => {
                                    println!(
                                        "{}{}",
                                        preview_prefix, "Cannot display preview: non-UTF-8 content"
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // If path is dir, then recurse into directories
            if path.is_dir() {
                let additional_prefix = if is_last { "    " } else { continuation };
                generate_tree(
                    &path,
                    &format!("{}{}", prefix, additional_prefix),
                    file_extension,
                    output_file,
                    depth + 1,
                    max_depth,
                    ignore_hidden,
                    show_size,
                    branch_style,
                    preview_lines,
                    date_filter,
                    size_min,
                    size_max,
                    include,
                    exclude,
                    icons,
                );
            }
        }
    }
}

pub fn generate_json_tree(
    path: &Path,
    ignore_hidden: bool,
    root_dir_name: &str,
    file_extension: Option<&String>,
    size_min: Option<u64>,
    size_max: Option<u64>,
    date_filter: Option<&String>,
    include: Option<&String>,
    exclude: Option<&String>,
) -> TreeNode {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| root_dir_name.to_string());

    let size_str = if path.is_dir() {
        format!("{:.2} KB", get_directory_size(path) as f64 / 1024.0)
    } else {
        metadata(path)
            .map(|meta| format!("{:.2} KB", meta.len() as f64 / 1024.0))
            .unwrap_or("size unknown".to_string())
    };

    let permission_str = metadata(path)
        .map(|meta| format!("{:o}", meta.permissions().mode()))
        .unwrap_or("permission unknown".to_string());

    let last_modification_date_str = get_human_readable_date(&path);

    let mut node = TreeNode {
        name,
        size: size_str,
        node_type: if path.is_dir() {
            "Directory".to_string()
        } else {
            "File".to_string()
        },
        permission: permission_str,
        last_modification_date: last_modification_date_str,
        children: vec![],
    };

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();

                // Apply ignore hidden filter
                if ignore_hidden && file_name.starts_with('.') {
                    continue;
                }
                // Check file extension when file extension have value
                // This is check for directory for file extension
                if path.is_dir() && file_extension.is_some() {
                    if !contains_matching_files_extension(&path, file_extension, ignore_hidden) {
                        continue;
                    }
                }

                // This is check for a file
                if path.is_file() {
                    if let Some(ext) = file_extension {
                        if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some(ext)
                        {
                            continue;
                        }
                    }
                }

                // Apply size filter
                if let Some(size) = metadata(&path).map(|meta| meta.len()).ok() {
                    if let Some(min) = size_min {
                        if size < min {
                            continue;
                        }
                    }
                    if let Some(max) = size_max {
                        if size > max {
                            continue;
                        }
                    }
                }

                // Apply include/exclude patterns
                if let Some(include_pattern) = include {
                    let re = Regex::new(include_pattern).unwrap();
                    if !re.is_match(&file_name) {
                        continue;
                    }
                }
                if let Some(exclude_pattern) = exclude {
                    let re = Regex::new(exclude_pattern).unwrap();
                    if re.is_match(&file_name) {
                        continue;
                    }
                }

                // Apply date filter
                if let Some(date_filter) = date_filter {
                    if !apply_date_filter(&path, date_filter) {
                        continue;
                    }
                }

                // Recursively build child nodes
                node.children.push(generate_json_tree(
                    &path,
                    ignore_hidden,
                    root_dir_name,
                    file_extension,
                    size_min,
                    size_max,
                    date_filter,
                    include,
                    exclude,
                ));
            }
        }
    }

    node
}

fn get_directory_size(path: &Path) -> u64 {
    let mut total_size: u64 = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    total_size += get_directory_size(&entry_path);
                } else {
                    total_size += match fs::metadata(&entry_path) {
                        Ok(metadata) => metadata.len(),
                        Err(_) => 0,
                    }
                }
            }
        }
    }

    total_size
}
