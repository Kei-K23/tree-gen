use std::{
    fs::{self, metadata},
    path::Path,
};

use colored::Colorize;
use serde::Serialize;

use crate::lib::filter::contains_matching_files_extension;

use super::output_file::write_output;

#[derive(Serialize)]
pub struct TreeNode {
    name: String,
    size: String,
    node_type: String,
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

            // Use color for better visualization
            let file_name_colored = if path.is_dir() {
                file_name.green()
            } else {
                file_name.normal()
            };

            let content = format!("{}{}{}{}", prefix, new_prefix, file_name_colored, size_str);

            // If output file exist, then write to file instead of printing out to terminal
            if let Some(output) = output_file {
                write_output(&output, &content).expect("Failed to write to file");
            } else {
                println!("{}", content);
            }

            // If path is dir, then recurse into directories
            if path.is_dir() {
                let additional_prefix = if is_last { "    " } else { "│   " };
                generate_tree(
                    &path,
                    &format!("{}{}", prefix, additional_prefix),
                    file_extension,
                    output_file,
                    depth + 1,
                    max_depth,
                    ignore_hidden,
                    show_size,
                );
            }
        }
    }
}

pub fn generate_json_tree(path: &Path, ignore_hidden: bool, root_dir_name: &str) -> TreeNode {
    let name = match path.file_name() {
        Some(path_name) => path_name.to_string_lossy().into_owned(),
        None => root_dir_name.to_string(),
    };

    // If show size flags is true, then get the file size from metadata
    let size_str = match metadata(&path) {
        // Convert to KB by divided by 1024
        Ok(metadata) => format!("{:.2} KB", metadata.len() as f64 / 1024.0),
        Err(_) => String::from("size unknown"),
    };

    let mut node = TreeNode {
        name,
        size: size_str,
        node_type: if path.is_dir() {
            "Directory".to_string()
        } else {
            "File".to_string()
        },
        children: vec![],
    };

    if path.is_dir() {
        if let Ok(entires) = fs::read_dir(path) {
            for entry in entires.filter_map(Result::ok) {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();

                // Check file for ignore hidden
                if ignore_hidden && file_name.starts_with(".") {
                    continue;
                }

                node.children
                    .push(generate_json_tree(&path, ignore_hidden, root_dir_name));
            }
        }
    }

    node
}
