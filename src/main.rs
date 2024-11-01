use std::{
    fs::{self, metadata},
    path::Path,
};

use clap::{Arg, ArgAction, Command};

/// Simple CLI tool to generate folder structure in ASCII for markdown files.
fn main() {
    // CLI interface
    let matches = Command::new("tree_gen")
        .version("0.1.0")
        .about("tree-gen is a CLI tool to generate folder structure in ASCII")
        .author("Kei-K23")
        .arg(
            Arg::new("path")
                .help("Path of the directory to display")
                .value_name("PATH")
                .required(true),
        )
        .arg(
            Arg::new("depth")
                .help("Maximum depth of the tree")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .default_value("10"),
        )
        .arg(
            Arg::new("file_extension")
                .help("Filter output to show only files with a specific file extension")
                .short('e')
                .value_name("EXT")
                .long("extension"),
        )
        .arg(
            Arg::new("ignore_hidden")
                .help("Ignore hidden files and folders")
                .short('i')
                .long("ignore-hidden")
                .required(false)
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show_sizes")
                .help("Show file size next to file name")
                .short('s')
                .long("show-sizes")
                .required(false)
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let path_str = matches.get_one::<String>("path").unwrap();
    let filter_extension = matches.get_one::<String>("file_extension");
    let depth_str = matches.get_one::<String>("depth").unwrap();
    let depth_int = depth_str.parse::<usize>().unwrap();
    let ignore_hidden = matches.get_one::<bool>("ignore_hidden").unwrap();
    let show_size = matches.get_one::<bool>("show_sizes").unwrap();

    let path = Path::new(path_str);
    let max_depth = Some(depth_int);

    // Print root directory (without prefix) if it has matching files
    if path.is_dir()
        && contains_matching_files_extension(path, filter_extension, ignore_hidden.to_owned())
    {
        println!(
            "{}",
            path.file_name()
                .unwrap_or_else(|| path.as_os_str())
                .to_string_lossy()
        );
        // Start the recursive tree generation for subdirectories
        generate_tree(
            path,
            "",
            filter_extension,
            1,
            max_depth,
            ignore_hidden.to_owned(),
            show_size.to_owned(),
        );
    }
}

/// Check if a directory contains files with the specified extension (for filtering)
fn contains_matching_files_extension(
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

/// Generate an ASCII representation of the directory structure.
fn generate_tree(
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
