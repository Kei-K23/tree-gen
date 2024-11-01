use std::{fs, path::Path};

use clap::{Arg, ArgAction, Command};

/// Simple CLI tool to generate folder structure in ASCII for markdown files.
fn main() {
    // CLI interface
    let matches = Command::new("tree_gen")
        .version("0.1.0")
        .about("tree-gen is a CLI tool to generate folder structure in ASCII for markdown files.")
        .author("Kei-K23")
        .arg(
            Arg::new("path")
                .help("Path of the directory to display")
                .required(true),
        )
        .arg(
            Arg::new("depth")
                .help("Maximum depth of the tree")
                .short('d')
                .long("depth")
                .default_value("10"),
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
        .get_matches();

    let path_str = matches.get_one::<String>("path").unwrap();
    let depth_str = matches.get_one::<String>("depth").unwrap();
    let depth_int = depth_str.parse::<usize>().unwrap();
    let ignore = matches.get_one::<bool>("ignore_hidden").unwrap();

    let path = Path::new(path_str);
    let max_depth = Some(depth_int);
    let dir_path = Path::new(path);

    match dir_path.file_name() {
        Some(name) => println!("{}", name.to_string_lossy()),
        None => println!("No root directory name detected"),
    }

    generate_tree(path, "", 1, max_depth, ignore.to_owned());
}

fn generate_tree(
    path: &Path,
    prefix: &str,
    depth: usize,
    max_depth: Option<usize>,
    ignore_hidden: bool,
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

            let is_last = i == entries.len() - 1;
            let new_prefix = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, new_prefix, file_name);

            // If path is dir, then recurse into directories
            if path.is_dir() {
                let additional_prefix = if is_last { "    " } else { "│   " };
                generate_tree(
                    &path,
                    &format!("{}{}", prefix, additional_prefix),
                    depth + 1,
                    max_depth,
                    ignore_hidden,
                );
            }
        }
    }
}
