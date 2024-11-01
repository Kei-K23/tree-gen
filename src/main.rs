use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use std::{env, path::Path};

// My Library modules
mod lib;
use lib::{
    filter::contains_matching_files_extension,
    generate::{generate_json_tree, generate_tree},
};

/// Simple CLI tool to generate folder structure in ASCII for markdown files.
fn main() {
    // CLI interface
    let matches = Command::new("tree_gen")
        .version("0.1.0")
        .about("tree-gen is a CLI tool to generate folder structure in ASCII and visualize folder structure")
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
            Arg::new("output_file")
                .help("Write the output to a file instead of printing to terminal")
                .short('o')
                .long("output")
                .value_name("FILE"),
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
        .arg(
            Arg::new("json")
                .help("Generate the json output of directory structure")
                .short('j')
                .long("json")
                .required(false)
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let path_str = matches.get_one::<String>("path").unwrap();
    let filter_extension = matches.get_one::<String>("file_extension");
    let output_file = matches.get_one::<String>("output_file");
    let depth_str = matches.get_one::<String>("depth").unwrap();
    let depth_int = depth_str.parse::<usize>().unwrap();
    let ignore_hidden = matches.get_one::<bool>("ignore_hidden").unwrap();
    let show_size = matches.get_one::<bool>("show_sizes").unwrap();
    let json = matches.get_one::<bool>("json").unwrap();

    let path = Path::new(path_str);
    let current_dir_path = env::current_dir().unwrap();
    let max_depth = Some(depth_int);

    // Get the name of the root directory for the display
    let root_dir_name = current_dir_path
        .file_name()
        .or_else(|| path.file_name())
        .unwrap_or_else(|| path.as_os_str())
        .to_string_lossy()
        .green()
        .to_string();

    println!("{}", root_dir_name);

    if *json {
        let json_tree = generate_json_tree(path, ignore_hidden.to_owned(), &root_dir_name);
        let json_tree_output =
            serde_json::to_string_pretty(&json_tree).expect("Failed to serialize the JSON");

        println!("{}", json_tree_output)
    } else {
        // Start the recursive tree generation for subdirectories
        generate_tree(
            path,
            "",
            filter_extension,
            output_file,
            1,
            max_depth,
            ignore_hidden.to_owned(),
            show_size.to_owned(),
        );
    }
}
